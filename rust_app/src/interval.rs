use time::OffsetDateTime;

#[derive(Debug, PartialEq, Eq)]
pub struct Interval {
    pub start: OffsetDateTime,
    pub end: OffsetDateTime,
}

pub fn interval(start: OffsetDateTime, end: OffsetDateTime) -> Option<Interval> {
    if start < end {
        Some(Interval { start, end })
    } else {
        None
    }
}

pub fn intersection(interval1: Interval, interval2: Interval) -> Option<Interval> {
    let start = interval1.start.max(interval2.start);
    let end = interval1.end.min(interval2.end);
    interval(start, end)
}

#[cfg(test)]
mod interval_tests {
    use time::Duration;
    use super::*;

    fn intvl(now: OffsetDateTime, start_hours: i64, end_hours: i64) -> Interval {
        interval(now + Duration::hours(start_hours), now + Duration::hours(end_hours)).unwrap()
    }

    #[test]
    fn interval_tests() {
        let now = OffsetDateTime::now_utc();
        assert_eq!(
            intersection(intvl(now, 0, 2), intvl(now, 2, 4)),
            None
        );
        assert_eq!(
            intersection(intvl(now, 0, 2), intvl(now, 1, 3)),
            Some(intvl(now, 1, 2))
        );
    }
}
