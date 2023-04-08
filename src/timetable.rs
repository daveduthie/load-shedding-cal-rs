pub fn column(day_of_month: i32) -> i32 {
    (day_of_month - 1) % 16
}

pub fn offset(day_of_month: i32) -> i32 {
    column(day_of_month) * 12
}

pub fn shift(day_of_month: i32) -> i32 {
    column(day_of_month).div_euclid(4)
}

pub fn initial_zones_for_stage(stage: usize) -> Vec<i32> {
    let zone_ids = vec![0, 8, 12, 4, 1, 9, 13, 5];
    zone_ids.into_iter().take(stage).collect()
}

#[cfg(test)]
mod timetable_tests {
    use super::*;

    #[test]
    fn column_tests() {
        assert_eq!(
            (1..=31).map(|day| column(day)).collect::<Vec<i32>>(),
            vec![
                0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9,
                10, 11, 12, 13, 14
            ]
        );
    }

    #[test]
    fn offset_tests() {
        assert_eq!(
            (1..=31).map(|day| offset(day)).collect::<Vec<i32>>(),
            vec![
                0, 12, 24, 36, 48, 60, 72, 84, 96, 108, 120, 132, 144, 156, 168, 180, 0, 12, 24,
                36, 48, 60, 72, 84, 96, 108, 120, 132, 144, 156, 168
            ]
        )
    }

    #[test]
    fn shift_tests() {
        assert_eq!(
            (1..=31).map(|day| shift(day)).collect::<Vec<i32>>(),
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
                .collect::<Vec<Vec<i32>>>(),
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
}
