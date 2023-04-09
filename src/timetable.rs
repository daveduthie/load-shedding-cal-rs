use time::{Duration, OffsetDateTime, Time};

use crate::interval::{self, Interval};

fn column(day_of_month: usize) -> usize {
    (day_of_month - 1) % 16
}

fn offset(day_of_month: usize) -> usize {
    column(day_of_month) * 12
}

fn shift(day_of_month: usize) -> usize {
    column(day_of_month).div_euclid(4)
}

fn initial_zones_for_stage(stage: usize) -> Vec<usize> {
    let zone_ids = vec![0, 8, 12, 4, 1, 9, 13, 5];
    zone_ids.into_iter().take(stage).collect()
}

fn schedule(stage: usize, day_of_month: usize) -> Vec<Vec<usize>> {
    let zone_seq = (1..=16)
        .cycle()
        .skip((shift(day_of_month) + offset(day_of_month)) as usize);

    let zone_seqs: Vec<Vec<usize>> = initial_zones_for_stage(stage)
        .iter()
        .map(|zone_id| zone_seq.clone().skip(*zone_id).take(12).collect())
        .collect();

    let mut result = Vec::new();

    for i in (0..12).into_iter() {
        let mut value: Vec<usize> = zone_seqs.iter().map(|zs| zs[i]).collect();
        value.sort();
        result.push(value)
    }

    result
}

pub fn timetable_for_stage_and_zone(stage: usize, zone_id: usize, now: OffsetDateTime) -> Vec<Interval> {
    let day_of_month = now.day() as usize;
    (0..24)
        .step_by(2)
        .zip(schedule(stage, day_of_month))
        .filter_map(|(hour, zones)| {
            if zones.contains(&zone_id) {
                let start = now.replace_time(Time::from_hms(hour, 0, 0).unwrap());
                let end = start + Duration::hours(2) + Duration::minutes(30);
                interval::interval(start, end)
            } else {
                None
            }
        })
        .collect()
}

#[cfg(test)]
mod timetable_tests {
    use time::macros::offset;

    use super::*;

    #[test]
    fn column_tests() {
        assert_eq!(
            (1..=31).map(|day| column(day)).collect::<Vec<usize>>(),
            vec![
                0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9,
                10, 11, 12, 13, 14
            ]
        );
    }

    #[test]
    fn offset_tests() {
        assert_eq!(
            (1..=31).map(|day| offset(day)).collect::<Vec<usize>>(),
            vec![
                0, 12, 24, 36, 48, 60, 72, 84, 96, 108, 120, 132, 144, 156, 168, 180, 0, 12, 24,
                36, 48, 60, 72, 84, 96, 108, 120, 132, 144, 156, 168
            ]
        )
    }

    #[test]
    fn shift_tests() {
        assert_eq!(
            (1..=31).map(|day| shift(day)).collect::<Vec<usize>>(),
            vec![
                0, 0, 0, 0, 1, 1, 1, 1, 2, 2, 2, 2, 3, 3, 3, 3, 0, 0, 0, 0, 1, 1, 1, 1, 2, 2, 2, 2,
                3, 3, 3
            ]
        )
    }

    #[test]
    fn initial_zones_for_stage_tests() {
        assert_eq!(
            (1..=8)
                .map(|day| initial_zones_for_stage(day))
                .collect::<Vec<Vec<usize>>>(),
            vec![
                vec![0],
                vec![0, 8],
                vec![0, 8, 12],
                vec![0, 8, 12, 4],
                vec![0, 8, 12, 4, 1],
                vec![0, 8, 12, 4, 1, 9],
                vec![0, 8, 12, 4, 1, 9, 13],
                vec![0, 8, 12, 4, 1, 9, 13, 5]
            ]
        )
    }

    #[test]
    fn schedule_tests() {
        assert_eq!(
            schedule(8, 2),
            vec![
                vec![1, 2, 5, 6, 9, 10, 13, 14],
                vec![2, 3, 6, 7, 10, 11, 14, 15],
                vec![3, 4, 7, 8, 11, 12, 15, 16],
                vec![1, 4, 5, 8, 9, 12, 13, 16],
                vec![1, 2, 5, 6, 9, 10, 13, 14],
                vec![2, 3, 6, 7, 10, 11, 14, 15],
                vec![3, 4, 7, 8, 11, 12, 15, 16],
                vec![1, 4, 5, 8, 9, 12, 13, 16],
                vec![1, 2, 5, 6, 9, 10, 13, 14],
                vec![2, 3, 6, 7, 10, 11, 14, 15],
                vec![3, 4, 7, 8, 11, 12, 15, 16],
                vec![1, 4, 5, 8, 9, 12, 13, 16]
            ]
        )
    }

    #[test]
    fn timetable_for_zone_tests() {
        let now = OffsetDateTime::now_utc().to_offset(offset!(+2));
        let timetable = timetable_for_stage_and_zone(3, 2, now);
        assert_eq!(
            timetable,
            vec![
                interval::interval(
                    now.replace_time(Time::from_hms(0, 0, 0).unwrap()),
                    now.replace_time(Time::from_hms(2, 30, 0).unwrap())
                )
                .unwrap(),
                interval::interval(
                    now.replace_time(Time::from_hms(8, 0, 0).unwrap()),
                    now.replace_time(Time::from_hms(10, 30, 0).unwrap())
                )
                .unwrap(),
            ]
        )
    }
}
