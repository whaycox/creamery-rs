use super::*;

/// An enum to represent the various fields of a Cron Expression.
#[derive(Debug, Clone, PartialEq)]
pub enum CronFieldType {
    /// The minute field.
    Minute,
    /// The hour field.
    Hour,
    /// The day of month field.
    DayOfMonth,
    /// The month field.
    Month,
    /// The day of week field.
    DayOfWeek,
}

impl Display for CronFieldType {   
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CronFieldType::Minute => write!(formatter, "minute"),
            CronFieldType::Hour => write!(formatter, "hour"),
            CronFieldType::DayOfMonth => write!(formatter, "day of month"),
            CronFieldType::Month => write!(formatter, "month"),
            CronFieldType::DayOfWeek => write!(formatter, "day of week"),
        }
    }
}

impl CronFieldType {
    pub fn fetch<TTimezone>(&self, datetime: &DateTime<TTimezone>) -> u32
    where TTimezone : TimeZone {
        match self {
            CronFieldType::Minute => datetime.minute(),
            CronFieldType::Hour => datetime.hour(),
            CronFieldType::DayOfMonth => datetime.day(),
            CronFieldType::Month => datetime.month(),
            CronFieldType::DayOfWeek => {
                match datetime.weekday() {
                    Weekday::Sun => 0,
                    Weekday::Mon => 1,
                    Weekday::Tue => 2,
                    Weekday::Wed => 3,
                    Weekday::Thu => 4,
                    Weekday::Fri => 5,
                    Weekday::Sat => 6,
                }
            },
        }
    }

    pub fn min(&self) -> u32 {
        match self {
            CronFieldType::Minute | CronFieldType::Hour | CronFieldType::DayOfWeek => 0,
            CronFieldType::DayOfMonth | CronFieldType::Month => 1,
        }
    }

    pub fn max(&self) -> u32 {
        match self {
            CronFieldType::Minute => 59,
            CronFieldType::Hour => 23,
            CronFieldType::DayOfMonth => 31,
            CronFieldType::Month => 12,
            CronFieldType::DayOfWeek => 6,
        }
    }

    pub fn translate<'a>(&self, value: &'a str) -> &'a str {
        match self {
            CronFieldType::Month => match value.to_lowercase().as_str() {
                "jan" => "1",
                "feb" => "2",
                "mar" => "3",
                "apr" => "4",
                "may" => "5",
                "jun" => "6",
                "jul" => "7",
                "aug" => "8",
                "sep" => "9",
                "oct" => "10",
                "nov" => "11",
                "dec" => "12",
                _ => value,
            },
            CronFieldType::DayOfWeek => match value.to_lowercase().as_str() {
                "sun" => "0",
                "mon" => "1",
                "tue" => "2",
                "wed" => "3",
                "thu" => "4",
                "fri" => "5",
                "sat" => "6",
                _ => value,
            }
            _ => value,
        }
    }
}
