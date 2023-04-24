use std::str::FromStr;

use anyhow::Result;
use regex::Regex;
use scraper::{Html, Selector};
use time::{
    macros::{format_description, offset},
    Date, Duration, Month, OffsetDateTime, Time, UtcOffset,
};
use tracing::info;

const URL: &str = "https://www.capetown.gov.za/Family%20and%20home/Residential-utility-services/Residential-electricity-services/Load-shedding-and-outages";

pub async fn get_schedule() -> Result<String> {
    Ok(reqwest::get(URL).await?.text().await?)
}

fn now() -> OffsetDateTime {
    OffsetDateTime::now_utc().to_offset(UtcOffset::from_hms(2, 0, 0).ok().unwrap())
}

fn try_parse_date(text: &str) -> Option<Date> {
    let re = Regex::new(r"(\d{1,2}) (\w+)\s*$").unwrap();
    let groups = re.captures(text)?;
    let day = groups.get(1)?.as_str().parse().ok()?;
    let month = Month::from_str(groups.get(2)?.as_str()).ok()?;
    // FIXME: wrong when dates overlap January 1
    Date::from_calendar_date(now().year(), month, day).ok()
}

#[cfg(test)]
mod try_parse_date_tests {
    use super::{now, try_parse_date};
    use time::{Date, Month};

    #[test]
    fn parse_dates() {
        let now_year = now().year();
        assert_eq!(
            try_parse_date("1 March"),
            Date::from_calendar_date(now_year, Month::March, 1).ok()
        );
        assert_eq!(
            try_parse_date("31 December"),
            Date::from_calendar_date(now_year, Month::December, 31).ok()
        )
    }
}

fn try_parse_time_range(text: &str) -> Option<(usize, Time, Time)> {
    let re = Regex::new(
        r"Stage (\d)(?: \(no load-shedding\))?: (underway until|\d{2}:\d{2}) (?:- )?(\d{2}:\d{2})",
    )
    .unwrap();
    let groups = re.captures(text)?;
    let stage = groups.get(1)?.as_str().parse().ok()?;
    let fmt = format_description!("[hour]:[minute]");
    let start = match &groups.get(2)?.as_str() {
        &"underway until" => now().time().replace_millisecond(0).unwrap(),
        s => Time::parse(s, &fmt).ok()?,
    };

    let end = Time::parse(groups.get(3)?.as_str(), &fmt).ok()?;

    Some((stage, start, end))
}

#[cfg(test)]
mod try_parse_time_range_tests {
    use super::{now, try_parse_time_range};
    use time::Time;

    #[test]
    fn parse_time_ranges() {
        assert_eq!(
            try_parse_time_range("Stage 4: 05:00 - 22:00"),
            Some((
                4,
                Time::from_hms(5, 0, 0).unwrap(),
                Time::from_hms(22, 0, 0).unwrap()
            ))
        );
        assert_eq!(
            try_parse_time_range("Stage 4: underway until 22:00"),
            Some((
                4,
                now().time().replace_millisecond(0).unwrap(),
                Time::from_hms(22, 0, 0).unwrap()
            ))
        )
    }
}

#[derive(Debug)]
pub struct LoadShedTime {
    pub start: OffsetDateTime,
    pub end: OffsetDateTime,
    pub stage: usize,
    pub guess: bool,
}

impl LoadShedTime {
    pub fn new(start: OffsetDateTime, end: OffsetDateTime, stage: usize, guess: bool) -> Self {
        let end = if end <= start {
            end + Duration::DAY
        } else {
            end
        };
        Self {
            start,
            end,
            stage,
            guess,
        }
    }

    pub fn from_date_and_times(date: Date, stage: usize, start_time: Time, end_time: Time) -> Self {
        let offset = offset!(+2);
        let start = date.with_time(start_time).assume_offset(offset);
        let end = date.with_time(end_time).assume_offset(offset);
        Self::new(start, end, stage, false)
    }

    pub fn title(&self) -> String {
        let mut s = String::new();
        if self.guess {
            s.push_str("[?] ")
        }
        s.push_str(&format!("Load shedding ({})", self.stage));
        s
    }
}

pub fn parse_html(html: &str) -> Vec<LoadShedTime> {
    let mut res = Vec::new();
    let mut curr_date = None;
    let div_sel = Selector::parse("div.section-pull").unwrap();

    for line in Html::parse_document(html)
        .select(&div_sel)
        .next()
        .expect("Could not find load shed times")
        .text()
    {
        if let Some(date) = try_parse_date(line) {
            info!(date = date.to_string(), "Found date");
            curr_date = Some(date)
        }
        if let Some((stage, start, end)) = try_parse_time_range(line) {
            info!(stage = stage, start = start.to_string(), end = end.to_string(), "Found time range");
            res.push(LoadShedTime::from_date_and_times(
                curr_date.expect("Expected date before time range"),
                stage,
                start,
                end,
            ));
        }
    }

    res
}

pub async fn schedule() -> Result<Vec<LoadShedTime>> {
    let mut times = parse_html(&get_schedule().await?);

    if let Some(LoadShedTime {
        start, end, stage, ..
    }) = times.last()
    {
        let t1 = start.replace_time(Time::MIDNIGHT) + Duration::DAY;
        let t2 = t1 + Duration::DAY;
        let t3 = t2 + Duration::DAY;
        let t4 = t3 + Duration::DAY;

        let midnight = LoadShedTime::new(*end, t1, *stage, true);
        let next_day = LoadShedTime::new(t2, t2, *stage, true);
        let day_after = LoadShedTime::new(t3, t3, *stage, true);
        let day_after_that = LoadShedTime::new(t4, t4, *stage, true);

        times.push(midnight);
        times.push(next_day);
        times.push(day_after);
        times.push(day_after_that);
    }

    Ok(times)
}
