use anyhow::Result;
use serde::Deserialize;
use time::{format_description::well_known::Iso8601, OffsetDateTime, PrimitiveDateTime, UtcOffset};

const URL: &str = "https://d42sspn7yra3u.cloudfront.net/coct-load-shedding-extended-status.json";

#[derive(Deserialize, Debug)]
struct LoadShedTimeRaw {
    start: String,
    end: String,
    stage: String,
}

#[derive(Debug)]
pub struct LoadShedTime {
    pub start: OffsetDateTime,
    pub end: OffsetDateTime,
    pub stage: usize,
}

fn parse_local_time(s: &str) -> Result<OffsetDateTime> {
    Ok(
        PrimitiveDateTime::parse(s, &Iso8601::DEFAULT)?
            .assume_offset(UtcOffset::from_hms(2, 0, 0)?),
    )
}

impl LoadShedTime {
    fn from_raw(record: &LoadShedTimeRaw) -> Result<Self> {
        Ok(LoadShedTime {
            start: parse_local_time(&record.start)?,
            end: parse_local_time(&record.end)?,
            stage: record.stage.parse()?,
        })
    }

    pub fn title(&self) -> String {
        let mut s = String::new();
        s.push_str(&format!("Load shedding ({})", self.stage));
        s
    }
}

pub async fn get_schedule() -> Result<Vec<LoadShedTime>> {
    Ok(reqwest::get(URL)
        .await?
        .json::<Vec<LoadShedTimeRaw>>()
        .await?
        .into_iter()
        .filter_map(|record| match LoadShedTime::from_raw(&record) {
            Ok(lst) => Some(lst),
            _ => {
                println!("Failed to parse: {:?}", record);
                None
            }
        })
        .collect())
}

pub async fn schedule() -> Result<Vec<LoadShedTime>> {
    Ok(get_schedule().await?)
}
