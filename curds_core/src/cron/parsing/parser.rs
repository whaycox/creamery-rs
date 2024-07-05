use super::*;
use handlers::*;

pub trait CronFieldParser {
    fn parse_minute(&self, field: &str) -> Result<CronField, CronParsingError>;
    fn parse_hour(&self, field: &str) -> Result<CronField, CronParsingError>;
    fn parse_day_of_month(&self, field: &str) -> Result<CronField, CronParsingError>;
    fn parse_month(&self, field: &str) -> Result<CronField, CronParsingError>;
    fn parse_day_of_week(&self, field: &str) -> Result<CronField, CronParsingError>;
}

pub mod testing {
    use super::*;
    use std::cell::RefCell;

    pub struct TestingCronFieldParser {
        parse_minute_calls: RefCell<u32>,
        parse_minute_inputs: RefCell<Vec<(String)>>,
        parse_minute_returns: RefCell<Vec<Result<CronField, CronParsingError>>>,
    }

    impl TestingCronFieldParser {
        pub fn new() -> Self {
            Self {
                parse_minute_calls: RefCell::new(0),
                parse_minute_inputs: RefCell::new(Vec::new()),
                parse_minute_returns: RefCell::new(Vec::new()),
            }
        }
    }

    impl CronFieldParser for TestingCronFieldParser {
        fn parse_minute(&self, field: &str) -> Result<CronField, CronParsingError> {
            let value = self.parse_minute_returns.borrow_mut().remove(0);
            *self.parse_minute_calls.borrow_mut() += 1;
            value
        }
    
        fn parse_hour(&self, field: &str) -> Result<CronField, CronParsingError> {
            todo!()
        }
    
        fn parse_day_of_month(&self, field: &str) -> Result<CronField, CronParsingError> {
            todo!()
        }
    
        fn parse_month(&self, field: &str) -> Result<CronField, CronParsingError> {
            todo!()
        }
    
        fn parse_day_of_week(&self, field: &str) -> Result<CronField, CronParsingError> {
            todo!()
        }
    }
}

static STANDARD_LINKS: OnceLock<CronValueParserLink> = OnceLock::new();
static DAY_OF_MONTH_LINKS: OnceLock<CronValueParserLink> = OnceLock::new();
static DAY_OF_WEEK_LINKS: OnceLock<CronValueParserLink> = OnceLock::new();
pub struct CurdsCronFieldParser;

impl CurdsCronFieldParser {
    fn parse_field(field: &str, links: &CronValueParserLink, field_type: CronFieldType) -> Result<CronField, CronParsingError> {
        let mut values: Vec<CronValue> = Vec::new(); 
        for value in field.split(",") {
            values.push(links.parse(&field_type, value)?)
        }
        Ok(CronField::new(field_type, values))
    }

    fn build_standard_links() -> CronValueParserLink {
        CronValueParserLink::tail(parse_single)
            .prepend(parse_wildcard)
            .prepend(parse_range)
    }
    fn build_day_of_month_links() -> CronValueParserLink {
        Self::build_standard_links()
            .prepend(parse_nearest_weekday)
            .prepend(parse_last_day_of_month)
    }
    fn build_day_of_week_links() -> CronValueParserLink {
        Self::build_standard_links()
            .prepend(parse_nth_day_of_week)
            .prepend(parse_last_day_of_week)
    }
}

impl CronFieldParser for CurdsCronFieldParser {
    fn parse_minute(&self, field: &str) -> Result<CronField, CronParsingError> {
        Self::parse_field(field, STANDARD_LINKS.get_or_init(Self::build_standard_links), CronFieldType::Minute)
    }

    fn parse_hour(&self, field: &str) -> Result<CronField, CronParsingError> {
        Self::parse_field(field, STANDARD_LINKS.get_or_init(Self::build_standard_links), CronFieldType::Hour)
    }

    fn parse_day_of_month(&self, field: &str) -> Result<CronField, CronParsingError> {
        Self::parse_field(field, DAY_OF_MONTH_LINKS.get_or_init(Self::build_day_of_month_links), CronFieldType::DayOfMonth)
    }

    fn parse_month(&self, field: &str) -> Result<CronField, CronParsingError> {
        Self::parse_field(field, STANDARD_LINKS.get_or_init(Self::build_standard_links), CronFieldType::Month)
    }

    fn parse_day_of_week(&self, field: &str) -> Result<CronField, CronParsingError> {
        Self::parse_field(field, DAY_OF_WEEK_LINKS.get_or_init(Self::build_day_of_week_links), CronFieldType::DayOfWeek)
    }
}