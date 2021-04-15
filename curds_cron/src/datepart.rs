use super::*;

/// An enum to represent the various parts of a DateTime against which a CronExpression can match.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CronDatePart {
    /// The DateTime's minutes value.
    Minutes,
    /// The DateTime's hours value.
    Hours,
    /// The DateTime's day value.
    DayOfMonth,
    /// The DateTime's month value.
    Month,
    /// The DateTime's weekday value.
    DayOfWeek,
}
impl Display for CronDatePart {   
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result<(), Error> {
        match self {
            CronDatePart::Minutes => write!(formatter, "Minutes"),
            CronDatePart::Hours => write!(formatter, "Hours"),
            CronDatePart::DayOfMonth => write!(formatter, "DayOfMonth"),
            CronDatePart::Month => write!(formatter, "Month"),
            CronDatePart::DayOfWeek => write!(formatter, "DayOfWeek"),
        }
    }
}
impl CronDatePart {
    /// Fetches the appropriate part of the DateTime.
    /// ```
    /// use chrono::{DateTime, Timelike, Utc};
    /// use curds_cron::CronDatePart;
    /// let part = CronDatePart::Minutes;
    /// let now = Utc::now();
    /// assert_eq!(now.minute(), part.fetch(&now));
    /// ```
    pub fn fetch<Tz>(&self, datetime: &DateTime<Tz>) -> u32
    where Tz : TimeZone {
        match self {
            CronDatePart::Minutes => datetime.minute(),
            CronDatePart::Hours => datetime.hour(),
            CronDatePart::DayOfMonth => datetime.day(),
            CronDatePart::Month => datetime.month(),
            CronDatePart::DayOfWeek => {
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

    /// Return the minimum allowable value for the part.
    /// ```
    /// use curds_cron::CronDatePart;
    /// let part = CronDatePart::Hours;
    /// assert_eq!(0, part.min());
    /// ```
    pub fn min(&self) -> u32 {
        match self {
            CronDatePart::Minutes | CronDatePart::Hours | CronDatePart::DayOfWeek => 0,
            CronDatePart::DayOfMonth | CronDatePart::Month => 1,
        }
    }

    /// Return the maximum allowable value for the part.
    /// ```
    /// use curds_cron::CronDatePart;
    /// let part = CronDatePart::Month;
    /// assert_eq!(12, part.max());
    /// ```
    pub fn max(&self) -> u32 {
        match self {
            CronDatePart::Minutes => 59,
            CronDatePart::Hours => 23,
            CronDatePart::DayOfMonth => 31,
            CronDatePart::Month => 12,
            CronDatePart::DayOfWeek => 6,
        }
    }

    /// Translate a value into its numeric representation, if possible.
    /// Will return the supplied string if it is not a recognized alias.
    /// ```
    /// use curds_cron::CronDatePart;
    /// let part = CronDatePart::DayOfWeek;
    /// assert_eq!("5", part.translate("Fri"));
    /// assert_eq!("foo", part.translate("foo"));
    /// ```
    pub fn translate<'a>(&self, value: &'a str) -> &'a str {
        match self {
            CronDatePart::Month => match value.to_lowercase().as_str() {
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
            CronDatePart::DayOfWeek => match value.to_lowercase().as_str() {
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
    use super::super::*;

    #[test]
    fn fetches_date_part() {
        let now = Utc::now();

        assert_eq!(now.minute(), CronDatePart::Minutes.fetch(&now));
        assert_eq!(now.hour(), CronDatePart::Hours.fetch(&now));
        assert_eq!(now.day(), CronDatePart::DayOfMonth.fetch(&now));
        assert_eq!(now.month(), CronDatePart::Month.fetch(&now));
    }

    #[test]
    fn fetches_day_of_week_part() {
        let sunday = "2021-04-04T00:00:00Z".parse::<DateTime<Utc>>().unwrap();
        
        assert_eq!(0, CronDatePart::DayOfWeek.fetch(&sunday));
        assert_eq!(1, CronDatePart::DayOfWeek.fetch(&(sunday + Duration::days(1))));
        assert_eq!(2, CronDatePart::DayOfWeek.fetch(&(sunday + Duration::days(2))));
        assert_eq!(3, CronDatePart::DayOfWeek.fetch(&(sunday + Duration::days(3))));
        assert_eq!(4, CronDatePart::DayOfWeek.fetch(&(sunday + Duration::days(4))));
        assert_eq!(5, CronDatePart::DayOfWeek.fetch(&(sunday + Duration::days(5))));
        assert_eq!(6, CronDatePart::DayOfWeek.fetch(&(sunday + Duration::days(6))));
    }

    #[test]
    fn min_is_expected() {
        assert_eq!(0, CronDatePart::Minutes.min());
        assert_eq!(0, CronDatePart::Hours.min());
        assert_eq!(1, CronDatePart::DayOfMonth.min());
        assert_eq!(1, CronDatePart::Month.min());
        assert_eq!(0, CronDatePart::DayOfWeek.min());
    }

    #[test]
    fn max_is_expected() {
        assert_eq!(59, CronDatePart::Minutes.max());
        assert_eq!(23, CronDatePart::Hours.max());
        assert_eq!(31, CronDatePart::DayOfMonth.max());
        assert_eq!(12, CronDatePart::Month.max());
        assert_eq!(6, CronDatePart::DayOfWeek.max());
    }

    #[test]
    fn months_translate() {
        assert_eq!("1", CronDatePart::Month.translate("JAN"));
        assert_eq!("2", CronDatePart::Month.translate("feb"));
        assert_eq!("3", CronDatePart::Month.translate("mAr"));
        assert_eq!("4", CronDatePart::Month.translate("Apr"));
        assert_eq!("5", CronDatePart::Month.translate("may"));
        assert_eq!("6", CronDatePart::Month.translate("jUN"));
        assert_eq!("7", CronDatePart::Month.translate("juL"));
        assert_eq!("8", CronDatePart::Month.translate("AUG"));
        assert_eq!("9", CronDatePart::Month.translate("SEP"));
        assert_eq!("10", CronDatePart::Month.translate("OCT"));
        assert_eq!("11", CronDatePart::Month.translate("nov"));
        assert_eq!("12", CronDatePart::Month.translate("dec"));
        assert_eq!("anything", CronDatePart::Month.translate("anything"));
    }

    #[test]
    fn days_of_week_translate() {
        assert_eq!("0", CronDatePart::DayOfWeek.translate("sun"));
        assert_eq!("1", CronDatePart::DayOfWeek.translate("MON"));
        assert_eq!("2", CronDatePart::DayOfWeek.translate("tue"));
        assert_eq!("3", CronDatePart::DayOfWeek.translate("Wed"));
        assert_eq!("4", CronDatePart::DayOfWeek.translate("tHu"));
        assert_eq!("5", CronDatePart::DayOfWeek.translate("frI"));
        assert_eq!("6", CronDatePart::DayOfWeek.translate("sat"));
        assert_eq!("anything", CronDatePart::DayOfWeek.translate("anything"));
    }

    #[test]
    fn other_translation_returns_self() {
        let anything = "anything";

        assert_eq!(anything, CronDatePart::Minutes.translate(anything));
        assert_eq!(anything, CronDatePart::Hours.translate(anything));
        assert_eq!(anything, CronDatePart::DayOfMonth.translate(anything));
    }
}