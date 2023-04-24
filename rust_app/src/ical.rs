use time::{format_description, macros::format_description, OffsetDateTime, UtcOffset};
use uuid::Uuid;

fn fmt_dt(date_time: OffsetDateTime) -> String {
    const FMT: &[format_description::FormatItem] =
        format_description!("[year][month][day]T[hour][minute][second]Z");
    date_time.to_offset(UtcOffset::UTC).format(&FMT).unwrap()
}

fn uid() -> String {
    String::from(
        Uuid::new_v4()
            .as_hyphenated()
            .encode_lower(&mut Uuid::encode_buffer()),
    )
}

fn event_(start: OffsetDateTime, end: OffsetDateTime, title: &str, uid: &str) -> String {
    let start_str = fmt_dt(start);
    let mut s = String::from("\nBEGIN:VEVENT");

    s.push_str("\nUID:");
    s.push_str(uid);

    s.push_str("\nDTSTAMP:");
    s.push_str(&start_str);

    s.push_str("\nDTSTART:");
    s.push_str(&start_str);

    s.push_str("\nDTEND:");
    s.push_str(&fmt_dt(end));

    s.push_str("\nSUMMARY:");
    s.push_str(title);

    s.push_str("\nEND:VEVENT");
    s
}

pub fn event(start: OffsetDateTime, end: OffsetDateTime, title: &str) -> String {
    event_(start, end, title, &uid())
}

pub fn ical(events: &[String]) -> String {
    let mut s = String::from(
        "BEGIN:VCALENDAR\nVERSION:2.0\nPRODID:-//io.github.daveduthie//load-shedding-calendar//EN",
    );
    for event in events {
        s.push_str(event);
    }
    s.push_str("\nEND:VCALENDAR");

    s
}

#[cfg(test)]
mod tests {
    use time::Duration;

    use super::*;

    #[test]
    fn event_test() {
        let now = OffsetDateTime::now_utc();
        let an_hours_time = now + Duration::HOUR;
        assert_eq!(
            event_(now, an_hours_time, "hi there", "id"),
            format!(
                r#"
BEGIN:VEVENT
UID:id
DTSTAMP:{}
DTSTART:{}
DTEND:{}
SUMMARY:hi there
END:VEVENT"#,
                fmt_dt(now),
                fmt_dt(now),
                fmt_dt(an_hours_time)
            )
        )
    }

    #[test]
    fn ical_test() {
        let now = OffsetDateTime::now_utc();
        let an_hours_time = now + Duration::HOUR;
        let ic = ical(&[event_(now, an_hours_time, "test event", "id")]);
        let expected = format!(
            r#"BEGIN:VCALENDAR
VERSION:2.0
PRODID:-//io.github.daveduthie//load-shedding-calendar//EN
BEGIN:VEVENT
UID:id
DTSTAMP:{}
DTSTART:{}
DTEND:{}
SUMMARY:test event
END:VEVENT
END:VCALENDAR"#,
            fmt_dt(now),
            fmt_dt(now),
            fmt_dt(an_hours_time)
        );
        println!("{}", &ic);

        assert_eq!(ic, expected);
    }
}
