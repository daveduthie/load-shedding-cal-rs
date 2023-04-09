use time::{format_description, macros::format_description, OffsetDateTime, UtcOffset};
use uuid::Uuid;

pub type IcalEventInput = (OffsetDateTime, OffsetDateTime, String);

fn format_date_time(date_time: OffsetDateTime) -> String {
    const FMT: &[format_description::FormatItem] =
        format_description!("[year][month][day]T[hour][second]Z");
    date_time.to_offset(UtcOffset::UTC).format(&FMT).unwrap()
}

pub fn event(start: OffsetDateTime, end: OffsetDateTime, title: &str) -> String {
    let start_str = format_date_time(start);

    let mut s = String::from("\nBEGIN:VEVENT");
    s.push_str("\nUID:");
    s.push_str(
        Uuid::new_v4()
            .as_hyphenated()
            .encode_lower(&mut Uuid::encode_buffer()),
    );
    s.push_str("\nDTSTAMP:");
    s.push_str(&start_str);

    s.push_str("\nDTSTART:");
    s.push_str(&start_str);

    s.push_str("\nDTEND:");
    s.push_str(&format_date_time(end));

    s.push_str("\nSUMMARY:");
    s.push_str(title);

    s.push_str("\nEND:VEVENT");

    s
}

pub fn ical(events: &[IcalEventInput]) -> String {
    let mut s = String::from(
        "BEGIN:VCALENDAR\nVERSION:2.0\nPRODID:-//io.github.daveduthie//load-shedding-calendar//EN",
    );
    for (start, end, title) in events {
        s.push_str(&event(*start, *end, title));
    }
    s.push_str("\nEND:VCALENDAR");

    s
}

// #[cfg(test)]
// mod tests {
//     use time::macros::offset;

//     use super::*;

//     #[test]
//     fn event_test() {
//         assert_eq!(
//             event(
//                 OffsetDateTime::UNIX_EPOCH,
//                 OffsetDateTime::now_utc(),
//                 "hi there"
//             ),
//             "\nBEGIN:VEVENT\nUID:d61952fb-50ef-449b-9fc2-ec3927b23d8b\nDTSTAMP:19700101T0000Z\nDTSTART:19700101T0000Z\nDTEND:20230408T1836Z"
//         )
//     }

//     #[test]
//     fn ical_test() {
//         let ic = ical(&vec![(
//             OffsetDateTime::UNIX_EPOCH,
//             OffsetDateTime::now_utc(),
//             String::from("test event"),
//         )]);
//         // println!("{:#?}", &ic);
//         assert_eq!(ic, String::from("foo"));
//     }
// }
