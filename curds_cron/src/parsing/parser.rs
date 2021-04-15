use super::*;

#[cfg_attr(test, automock)]
pub trait CronFieldParser {
    fn parse(date_part: CronDatePart, field: &str) -> Result<CronField, CronParsingError>;
}

#[cfg_attr(test, automock)]
pub trait CronValueParsingHandler {
    fn parse(&self, date_part: &CronDatePart, value: &str) -> Option<Result<CronValue, CronParsingError>>;
}

#[cfg_attr(test, automock)]
pub trait CronValueParserFactory {
    fn create(date_part: &CronDatePart) -> CronValueParserLink;
}

pub struct CurdsCronParser;
impl CronFieldParser for CurdsCronParser {   
    fn parse(date_part: CronDatePart, field: &str) -> Result<CronField, CronParsingError> { CurdsCronParser::parse_field::<CurdsCronValueParserFactory>(date_part, field) }
}
impl CurdsCronParser {
    fn parse_field<TValueParserFactory>(date_part: CronDatePart, field: &str) -> Result<CronField, CronParsingError>
    where TValueParserFactory : CronValueParserFactory  {
        let parser = TValueParserFactory::create(&date_part);
        let mut values = Vec::<CronValue>::new(); 
        for value in field.split(",") {
            values.push(parser.parse(&date_part, value)?)
        }
        Ok(CronField::new(date_part, values))
    }
}

struct CurdsCronValueParserFactory;
impl CronValueParserFactory for CurdsCronValueParserFactory {
    fn create(date_part: &CronDatePart) -> CronValueParserLink {
        let link = Self::default_links();
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
impl CurdsCronValueParserFactory {    
    fn default_links() -> CronValueParserLink {
        CronValueParserLink::tail(Box::new(SingleValueHandler))
            .prepend(Box::new(WildcardValueHandler))
            .prepend(Box::new(RangeValueHandler))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_field_parses_with_factory_parser() -> Result<(), CronParsingError> {
        let factory_context = MockCronValueParserFactory::create_context();
        factory_context.expect()
            .with(predicate::eq(CronDatePart::DayOfWeek))
            .times(1)
            .returning(|_| { 
                let mut sequence = Sequence::new();
                let mut mock_handler = MockCronValueParsingHandler::new();
                mock_handler.expect_parse()
                    .with(predicate::eq(CronDatePart::DayOfWeek), predicate::eq("One"))
                    .times(1)
                    .in_sequence(&mut sequence)
                    .returning(|_,_| { Some(Ok(CronValue::Any)) });
                mock_handler.expect_parse()
                    .with(predicate::eq(CronDatePart::DayOfWeek), predicate::eq("Two"))
                    .times(1)
                    .in_sequence(&mut sequence)
                    .returning(|_,_| { Some(Ok(CronValue::Any)) });
                mock_handler.expect_parse()
                    .with(predicate::eq(CronDatePart::DayOfWeek), predicate::eq("Three"))
                    .times(1)
                    .in_sequence(&mut sequence)
                    .returning(|_,_| { Some(Ok(CronValue::Any)) });
                CronValueParserLink::tail(Box::new(mock_handler)) 
            });

        CurdsCronParser::parse_field::<MockCronValueParserFactory>(CronDatePart::DayOfWeek, "One,Two,Three")?;
        Ok(())        
    }
}