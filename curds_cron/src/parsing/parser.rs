use super::super::*;
use super::parsing_handlers::*;

pub trait CronFieldParser {
    fn parse(date_part: CronDatePart, field: &str) -> CronField;
}

pub trait CronValueParsingHandler {
    fn parse(&self, date_part: &CronDatePart, value: &str) -> Option<CronValue>;
}

trait CronValueParserFactory {
    fn create(date_part: &CronDatePart) -> CronValueParserLink;
}

pub struct CurdsCronParser;
impl CronFieldParser for CurdsCronParser {   
    fn parse(date_part: CronDatePart, field: &str) -> CronField { CurdsCronParser::parse_field::<CurdsCronValueParserFactory>(date_part, field) }
}
impl CurdsCronParser {
    fn parse_field<TValueParserFactory>(date_part: CronDatePart, field: &str) -> CronField
    where TValueParserFactory : CronValueParserFactory  {
        let parser = TValueParserFactory::create(&date_part);
        let mut values = Vec::<CronValue>::new(); 
        for value in field.split(",") {
            values.push(parser.parse(&date_part, value))
        }
        CronField::new(date_part, values)
    }
}

struct CurdsCronValueParserFactory;
impl CronValueParserFactory for CurdsCronValueParserFactory {
    fn create(date_part: &CronDatePart) -> CronValueParserLink {
        let link = CronValueParserLink::default_links();
        match date_part {
            CronDatePart::DayOfMonth => return link
                .prepend(Box::new(NearestWeekdayValueHandler))
                .prepend(Box::new(LastDayOfMonthValueHandler)),
            CronDatePart::DayOfWeek => return link
                .prepend(Box::new(NthDayOfWeekValueHandler))
                .prepend(Box::new(LastDayOfWeekValueHandler)),
            _ => return link
        }
    }
}

struct CronValueParserLink {
    current: Box<dyn CronValueParsingHandler>,
    successor: Option<Box<CronValueParserLink>>,
}
impl CronValueParserLink {
    pub fn default_links() -> CronValueParserLink {
        CronValueParserLink::tail(Box::new(SingleValueHandler))
            .prepend(Box::new(WildcardValueHandler))
            .prepend(Box::new(RangeValueHandler))
    }

    pub fn tail(handler: Box<dyn CronValueParsingHandler>) -> CronValueParserLink {
        CronValueParserLink {
            current: handler,
            successor: None,
        }
    }

    pub fn prepend(self, handler: Box<dyn CronValueParsingHandler>) -> CronValueParserLink {
        CronValueParserLink {
            current: handler,
            successor: Some(Box::new(self)),
        }
    }

    pub fn parse(&self, date_part: &CronDatePart, value: &str) -> CronValue {
        if let Some(cron_value) = self.current.parse(date_part, value) {
            return cron_value;
        }
        else {        
            match &self.successor {
                None => panic!("Unsupported value: {}", value),
                Some(link) => return link.parse(date_part, value),
            }
        }
    }
}