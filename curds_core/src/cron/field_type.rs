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

#[cfg(test)]
mod tests {
    use super::*;
    use time::Utc;

    #[test]
    fn fetches_date_part() {
        let now = Utc::now();

        assert_eq!(now.minute(), CronFieldType::Minute.fetch(&now));
        assert_eq!(now.hour(), CronFieldType::Hour.fetch(&now));
        assert_eq!(now.day(), CronFieldType::DayOfMonth.fetch(&now));
        assert_eq!(now.month(), CronFieldType::Month.fetch(&now));
    }

    #[test]
    fn fetches_day_of_week_part() {
        let sunday = "2021-04-04T00:00:00Z".parse::<DateTime<Utc>>().unwrap();
        
        assert_eq!(0, CronFieldType::DayOfWeek.fetch(&sunday));
        assert_eq!(1, CronFieldType::DayOfWeek.fetch(&(sunday + Duration::days(1))));
        assert_eq!(2, CronFieldType::DayOfWeek.fetch(&(sunday + Duration::days(2))));
        assert_eq!(3, CronFieldType::DayOfWeek.fetch(&(sunday + Duration::days(3))));
        assert_eq!(4, CronFieldType::DayOfWeek.fetch(&(sunday + Duration::days(4))));
        assert_eq!(5, CronFieldType::DayOfWeek.fetch(&(sunday + Duration::days(5))));
        assert_eq!(6, CronFieldType::DayOfWeek.fetch(&(sunday + Duration::days(6))));
    }

    #[test]
    fn min_is_expected() {
        assert_eq!(0, CronFieldType::Minute.min());
        assert_eq!(0, CronFieldType::Hour.min());
        assert_eq!(1, CronFieldType::DayOfMonth.min());
        assert_eq!(1, CronFieldType::Month.min());
        assert_eq!(0, CronFieldType::DayOfWeek.min());
    }

    #[test]
    fn max_is_expected() {
        assert_eq!(59, CronFieldType::Minute.max());
        assert_eq!(23, CronFieldType::Hour.max());
        assert_eq!(31, CronFieldType::DayOfMonth.max());
        assert_eq!(12, CronFieldType::Month.max());
        assert_eq!(6, CronFieldType::DayOfWeek.max());
    }

    #[test]
    fn months_translate() {
        assert_eq!("1", CronFieldType::Month.translate("JAN"));
        assert_eq!("2", CronFieldType::Month.translate("feb"));
        assert_eq!("3", CronFieldType::Month.translate("mAr"));
        assert_eq!("4", CronFieldType::Month.translate("Apr"));
        assert_eq!("5", CronFieldType::Month.translate("may"));
        assert_eq!("6", CronFieldType::Month.translate("jUN"));
        assert_eq!("7", CronFieldType::Month.translate("juL"));
        assert_eq!("8", CronFieldType::Month.translate("AUG"));
        assert_eq!("9", CronFieldType::Month.translate("SEP"));
        assert_eq!("10", CronFieldType::Month.translate("OCT"));
        assert_eq!("11", CronFieldType::Month.translate("nov"));
        assert_eq!("12", CronFieldType::Month.translate("dec"));
        assert_eq!("anything", CronFieldType::Month.translate("anything"));
    }

    #[test]
    fn days_of_week_translate() {
        assert_eq!("0", CronFieldType::DayOfWeek.translate("sun"));
        assert_eq!("1", CronFieldType::DayOfWeek.translate("MON"));
        assert_eq!("2", CronFieldType::DayOfWeek.translate("tue"));
        assert_eq!("3", CronFieldType::DayOfWeek.translate("Wed"));
        assert_eq!("4", CronFieldType::DayOfWeek.translate("tHu"));
        assert_eq!("5", CronFieldType::DayOfWeek.translate("frI"));
        assert_eq!("6", CronFieldType::DayOfWeek.translate("sat"));
        assert_eq!("anything", CronFieldType::DayOfWeek.translate("anything"));
    }

    #[test]
    fn other_translation_returns_self() {
        let anything = "anything";

        assert_eq!(anything, CronFieldType::Minute.translate(anything));
        assert_eq!(anything, CronFieldType::Hour.translate(anything));
        assert_eq!(anything, CronFieldType::DayOfMonth.translate(anything));
    }
}