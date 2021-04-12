use super::*;

#[derive(Debug, PartialEq)]
pub enum CronValue {
    Any,
    Single(u32),
    Step(u32),
    Range { min: u32, max: u32 },
    LastDayOfMonth { offset: u32 },
    LastDayOfWeek { day_of_week: u32 },
    NthDayOfWeek { day_of_week: u32, n: u32 },
    NearestWeekday { day_of_month: u32 },
}
impl CronValue {
    pub fn is_match<Tz>(&self, date_part: &CronDatePart, datetime: &DateTime<Tz>) -> bool
    where Tz : TimeZone {
        let part = date_part.fetch(datetime);
        match &self {
            CronValue::Any => true,
            CronValue::Single(value) => value == &part,
            CronValue::Step(value) => (part - date_part.min()) % value == 0,
            CronValue::Range { min, max } => min <= &part && max >= &part,
            CronValue::LastDayOfMonth { offset } => part == CronValue::last_day_of_month(datetime) - offset,
            CronValue::LastDayOfWeek { day_of_week } => {
                let added_time = datetime.clone() + Duration::days(7);
                &part == day_of_week && added_time.month() != datetime.month()
            },
            CronValue::NthDayOfWeek { day_of_week, n } => {
                let occurrence = ((datetime.day() - 1) / 7) + 1;
                &part == day_of_week && &occurrence == n
            },
            CronValue::NearestWeekday { day_of_month } => {
                match datetime.weekday() {
                    Weekday::Sun | Weekday::Sat => false,
                    Weekday::Mon => &part == day_of_month || &part == &(day_of_month + 1),
                    Weekday::Fri => &part == day_of_month || &part == &(day_of_month - 1),
                    _ => &part == day_of_month,
                }
            }
        }
    }

    fn last_day_of_month<Tz>(datetime: &DateTime<Tz>) -> u32
    where Tz : TimeZone {
        match datetime.month() {
            1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
            4 | 6 | 9 | 11 => 30,
            2 => if datetime.year() % 4 == 0 { 29 }
            else { 28 },
            _ => panic!("{} isn't a valid month", datetime.month())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn any_matches_always() {
        let value = CronValue::Any;

        assert_eq!(true, value.is_match(&CronDatePart::Minutes, &Utc::now()));
    }

    macro_rules! test_value {
        ($test_name:ident => $value:expr, $part:expr => ($($test_date:expr => $expected:expr),+)) => {
            #[test]
            fn $test_name() {
                let test_object = $value;

                $(
                    assert_eq!($expected, test_object.is_match(&$part, &$test_date.parse::<DateTime<Utc>>().unwrap()),
                        "Expected {:?}({:?}) to fire {:?} for {:?}", $value, $part, $expected, $test_date);
                )+
            }
        }
    }
    
    test_value! {
        single_matches_on_part => CronValue::Single(5), CronDatePart::Month =>
        (
            "2021-04-01T00:00:00Z" => false,
            "2021-05-01T00:00:00Z" => true,
            "2021-06-01T00:00:00Z" => false
        )
    }
    
    test_value! {
        step_matches_from_zero_correctly => CronValue::Step(3), CronDatePart::Hours =>
        (
            "2021-04-01T00:00:00Z" => true,
            "2021-04-01T01:00:00Z" => false,
            "2021-04-01T02:00:00Z" => false,
            "2021-04-01T03:00:00Z" => true,
            "2021-04-01T04:00:00Z" => false,
            "2021-04-01T05:00:00Z" => false,
            "2021-04-01T06:00:00Z" => true
        )
    }

    test_value! {
        step_matches_from_one_correctly => CronValue::Step(2), CronDatePart::Month => 
        (
            "2021-01-01T00:00:00Z" => true,
            "2021-02-01T00:00:00Z" => false,
            "2021-03-01T00:00:00Z" => true,
            "2021-04-01T00:00:00Z" => false,
            "2021-05-01T00:00:00Z" => true
        )
    }

    test_value! {
        range_matches_inclusively_on_part => CronValue::Range { min: 5, max: 7 }, CronDatePart::Hours =>
        (
            "2021-04-01T04:00:00Z" => false,
            "2021-04-01T05:00:00Z" => true,
            "2021-04-01T06:00:00Z" => true,
            "2021-04-01T07:00:00Z" => true,
            "2021-04-01T08:00:00Z" => false
        )
    }

    test_value! {
        last_day_of_month_matches_without_offset => CronValue::LastDayOfMonth { offset: 0 }, CronDatePart::DayOfMonth =>
        (
            "2021-01-30T00:00:00Z" => false,
            "2021-01-31T00:00:00Z" => true,
            "2020-02-28T00:00:00Z" => false,
            "2020-02-29T00:00:00Z" => true,
            "2021-02-28T00:00:00Z" => true,
            "2021-04-29T00:00:00Z" => false,
            "2021-04-30T00:00:00Z" => true
        )
    }

    test_value! {
        last_day_of_month_matches_with_offset => CronValue::LastDayOfMonth { offset: 1 }, CronDatePart::DayOfMonth =>
        (
            "2021-01-30T00:00:00Z" => true,
            "2021-01-31T00:00:00Z" => false,
            "2020-02-28T00:00:00Z" => true,
            "2020-02-29T00:00:00Z" => false,
            "2021-02-28T00:00:00Z" => false,
            "2021-04-29T00:00:00Z" => true,
            "2021-04-30T00:00:00Z" => false
        )
    }

    test_value! {
        last_day_of_week_matches_correctly => CronValue::LastDayOfWeek { day_of_week: 1 }, CronDatePart::DayOfWeek =>
        (
            "2021-03-01T00:00:00Z" => false,
            "2021-03-08T00:00:00Z" => false,
            "2021-03-15T00:00:00Z" => false,
            "2021-03-22T00:00:00Z" => false,
            "2021-03-29T00:00:00Z" => true,
            "2021-04-05T00:00:00Z" => false,
            "2021-04-12T00:00:00Z" => false,
            "2021-04-19T00:00:00Z" => false,
            "2021-04-26T00:00:00Z" => true     
        )
    }

    test_value! {
        nth_day_of_week_matches_correctly => CronValue::NthDayOfWeek { day_of_week: 3, n: 2 }, CronDatePart::DayOfWeek =>
        (
            "2021-04-07T00:00:00Z" => false,
            "2021-04-14T00:00:00Z" => true,
            "2021-04-21T00:00:00Z" => false,
            "2021-04-28T00:00:00Z" => false
        )
    }

    test_value! {
        nearest_weekday_matches_correctly => CronValue::NearestWeekday { day_of_month: 4 }, CronDatePart::DayOfMonth =>
        (
            "2021-04-03T00:00:00Z" => false,
            "2021-04-04T00:00:00Z" => false,
            "2021-04-05T00:00:00Z" => true,
            "2021-04-06T00:00:00Z" => false,
            "2020-07-02T00:00:00Z" => false,
            "2020-07-03T00:00:00Z" => true,
            "2020-07-04T00:00:00Z" => false,
            "2020-07-05T00:00:00Z" => false,
            "2021-03-03T00:00:00Z" => false,
            "2021-03-04T00:00:00Z" => true,
            "2021-03-05T00:00:00Z" => false,
            "2021-01-03T00:00:00Z" => false,
            "2021-01-04T00:00:00Z" => true,
            "2021-01-05T00:00:00Z" => false,
            "2020-12-03T00:00:00Z" => false,
            "2020-12-04T00:00:00Z" => true,
            "2020-12-05T00:00:00Z" => false
        )
    }

    #[test]
    fn last_day_of_month_expected() {
        assert_eq!(31, CronValue::last_day_of_month(&"2021-01-01T00:00:00Z".parse::<DateTime<Utc>>().unwrap()));
        assert_eq!(29, CronValue::last_day_of_month(&"2020-02-01T00:00:00Z".parse::<DateTime<Utc>>().unwrap()));
        assert_eq!(28, CronValue::last_day_of_month(&"2021-02-01T00:00:00Z".parse::<DateTime<Utc>>().unwrap()));
        assert_eq!(31, CronValue::last_day_of_month(&"2021-03-01T00:00:00Z".parse::<DateTime<Utc>>().unwrap()));
        assert_eq!(30, CronValue::last_day_of_month(&"2021-04-01T00:00:00Z".parse::<DateTime<Utc>>().unwrap()));
        assert_eq!(31, CronValue::last_day_of_month(&"2021-05-01T00:00:00Z".parse::<DateTime<Utc>>().unwrap()));
        assert_eq!(30, CronValue::last_day_of_month(&"2021-06-01T00:00:00Z".parse::<DateTime<Utc>>().unwrap()));
        assert_eq!(31, CronValue::last_day_of_month(&"2021-07-01T00:00:00Z".parse::<DateTime<Utc>>().unwrap()));
        assert_eq!(31, CronValue::last_day_of_month(&"2021-08-01T00:00:00Z".parse::<DateTime<Utc>>().unwrap()));
        assert_eq!(30, CronValue::last_day_of_month(&"2021-09-01T00:00:00Z".parse::<DateTime<Utc>>().unwrap()));
        assert_eq!(31, CronValue::last_day_of_month(&"2021-10-01T00:00:00Z".parse::<DateTime<Utc>>().unwrap()));
        assert_eq!(30, CronValue::last_day_of_month(&"2021-11-01T00:00:00Z".parse::<DateTime<Utc>>().unwrap()));
        assert_eq!(31, CronValue::last_day_of_month(&"2021-12-01T00:00:00Z".parse::<DateTime<Utc>>().unwrap()));
    }
}