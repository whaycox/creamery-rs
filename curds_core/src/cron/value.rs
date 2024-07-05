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

impl Display for CronValue {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result<(), std::fmt::Error> { 
        match &self {
            CronValue::Any => write!(formatter, "*"),
            CronValue::Single(value) => write!(formatter, "{}", value),
            CronValue::Step(value) => write!(formatter, "*/{}", value),
            CronValue::Range { min, max } => write!(formatter, "{}-{}", min, max),
            CronValue::LastDayOfMonth { offset } => {
                if offset > &0 {
                    return write!(formatter, "L-{}", offset);
                }
                else {
                    return write!(formatter, "L");
                }
            },
            CronValue::LastDayOfWeek { day_of_week } => write!(formatter, "{}L", day_of_week),
            CronValue::NthDayOfWeek { day_of_week, n } => write!(formatter, "{}#{}", day_of_week, n),
            CronValue::NearestWeekday { day_of_month } => write!(formatter, "{}W", day_of_month),
        }
    }
}

impl CronValue {
    pub fn is_responsive<T: TimeZone>(&self, time: &DateTime<T>, field_type: &CronFieldType) -> bool {
        let part = field_type.fetch(time);
        match &self {
            CronValue::Any => true,
            CronValue::Single(value) => *value == part,
            CronValue::Step(value) => (part - field_type.min()) % value == 0,
            CronValue::Range { min, max } => *min <= part && *max >= part,
            CronValue::LastDayOfMonth { offset } => part == CronValue::last_day_of_month(time) - offset,
            CronValue::LastDayOfWeek { day_of_week } => {
                let added_time = time.clone() + Duration::days(7);
                part == *day_of_week && added_time.month() != time.month()
            },
            CronValue::NthDayOfWeek { day_of_week, n } => {
                let occurrence = ((time.day() - 1) / 7) + 1;
                part == *day_of_week && &occurrence == n
            },
            CronValue::NearestWeekday { day_of_month } => {
                match time.weekday() {
                    Weekday::Sun | Weekday::Sat => false,
                    Weekday::Mon => part == *day_of_month || part == day_of_month + 1,
                    Weekday::Fri => part == *day_of_month || part == day_of_month - 1,
                    _ => part == *day_of_month,
                }
            }
        }
    }

    fn last_day_of_month<T: TimeZone>(time: &DateTime<T>) -> u32 {
        match time.month() {
            1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
            4 | 6 | 9 | 11 => 30,
            2 => if time.year() % 4 == 0 { 29 }
            else { 28 },
            _ => panic!("{} isn't a valid month", time.month())
        }
    }
}