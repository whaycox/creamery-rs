use super::*;
use handlers::*;

pub trait CronFieldParser {
    fn parse_minute(field: &str) -> Result<CronField, CronParsingError>;
    fn parse_hour(field: &str) -> Result<CronField, CronParsingError>;
    fn parse_day_of_month(field: &str) -> Result<CronField, CronParsingError>;
    fn parse_month(field: &str) -> Result<CronField, CronParsingError>;
    fn parse_day_of_week(field: &str) -> Result<CronField, CronParsingError>;
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
    fn parse_minute(field: &str) -> Result<CronField, CronParsingError> {
        Self::parse_field(field, STANDARD_LINKS.get_or_init(Self::build_standard_links), CronFieldType::Minute)
    }

    fn parse_hour(field: &str) -> Result<CronField, CronParsingError> {
        Self::parse_field(field, STANDARD_LINKS.get_or_init(Self::build_standard_links), CronFieldType::Hour)
    }

    fn parse_day_of_month(field: &str) -> Result<CronField, CronParsingError> {
        Self::parse_field(field, DAY_OF_MONTH_LINKS.get_or_init(Self::build_day_of_month_links), CronFieldType::DayOfMonth)
    }

    fn parse_month(field: &str) -> Result<CronField, CronParsingError> {
        Self::parse_field(field, STANDARD_LINKS.get_or_init(Self::build_standard_links), CronFieldType::Month)
    }

    fn parse_day_of_week(field: &str) -> Result<CronField, CronParsingError> {
        Self::parse_field(field, DAY_OF_WEEK_LINKS.get_or_init(Self::build_day_of_week_links), CronFieldType::DayOfWeek)
    }
}