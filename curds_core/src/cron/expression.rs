use super::*;

#[derive(Debug)]
pub struct CronExpression {
    fields: [CronField; Self::FIELD_COUNT],
}

impl Display for CronExpression {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result<(), std::fmt::Error> { 
        write!(formatter, "{} {} {} {} {}", 
            &self.fields[0],
            &self.fields[1],
            &self.fields[2],
            &self.fields[3],
            &self.fields[4])?;
        Ok(())
    }
}

impl CronExpression {
    pub const FIELD_COUNT : usize = 5;
    const FIELD_DELIMITER : char = ' ';

    pub fn is_responsive<T: TimeZone>(&self, time: &DateTime<T>) -> bool {
        for field in &self.fields {
            if !field.is_responsive(time) {
                return false;
            }
        }
        true
    }

    pub fn parse<T: CronFieldParser>(expression: &str, parser: &T) -> Result<CronExpression, CronParsingError> {
        let parts: Vec<&str> = expression
            .split(Self::FIELD_DELIMITER)
            .filter(|part| part.len() > 0)
            .collect();
        if parts.len() != Self::FIELD_COUNT {
            return Err(CronParsingError::FieldCount {
                expression: expression.to_owned(), 
                parts: parts.len()
            });
        }
        Ok(CronExpression { 
            fields : [   
                parser.parse_minute(parts[0])?,
                parser.parse_hour(parts[1])?,
                parser.parse_day_of_month(parts[2])?,
                parser.parse_month(parts[3])?,
                parser.parse_day_of_week(parts[4])?,
            ]
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use time::Utc;

    fn test_parser() -> TestingCronFieldParser {
        let parser = TestingCronFieldParser::new();
        parser.default_return_parse_minute(|_| Ok(CronField::new(CronFieldType::Minute, vec![CronValue::Any])));
        parser.default_return_parse_hour(|_| Ok(CronField::new(CronFieldType::Hour, vec![CronValue::Any])));
        parser.default_return_parse_day_of_month(|_| Ok(CronField::new(CronFieldType::DayOfMonth, vec![CronValue::Any])));
        parser.default_return_parse_month(|_| Ok(CronField::new(CronFieldType::Month, vec![CronValue::Any])));
        parser.default_return_parse_day_of_week(|_| Ok(CronField::new(CronFieldType::DayOfWeek, vec![CronValue::Any])));

        parser
    }

    #[test]
    fn too_long_expression_fails() {
        match CronExpression::parse("* * * * * *", &test_parser()) {
            Err(CronParsingError::FieldCount { expression, parts }) => { 
                assert_eq!("* * * * * *", expression);
                assert_eq!(6, parts);
            },
            _ => panic!("Did not get the expected error"),
        }
    }

    #[test]
    fn too_short_expression_fails() {
        match CronExpression::parse("* * * *", &test_parser()) {
            Err(CronParsingError::FieldCount { expression, parts }) => { 
                assert_eq!("* * * *", expression);
                assert_eq!(4, parts);
            },
            _ => panic!("Did not get the expected error"),
        }
    }

    #[test]
    fn forwards_minute_parse_error() {
        let test_parser = test_parser();
        test_parser.store_return_parse_minute(|_| Err(CronParsingError::InvalidValue { value: "test_value".to_owned(), field_type: CronFieldType::Minute }), 1);
        match CronExpression::parse("* * * * *", &test_parser) {
            Err(CronParsingError::InvalidValue { value, field_type }) => { 
                assert_eq!("test_value", value);
                assert_eq!(CronFieldType::Minute, field_type);
            },
            _ => panic!("Did not get the expected error"),
        }
    }

    #[test]
    fn forwards_hour_parse_error() {
        let test_parser = test_parser();
        test_parser.store_return_parse_hour(|_| Err(CronParsingError::InvalidValue { value: "test_value".to_owned(), field_type: CronFieldType::Hour }), 1);
        match CronExpression::parse("* * * * *", &test_parser) {
            Err(CronParsingError::InvalidValue { value, field_type }) => { 
                assert_eq!("test_value", value);
                assert_eq!(CronFieldType::Hour, field_type);
            },
            _ => panic!("Did not get the expected error"),
        }
    }

    #[test]
    fn forwards_day_of_month_parse_error() {
        let test_parser = test_parser();
        test_parser.store_return_parse_day_of_month(|_| Err(CronParsingError::InvalidValue { value: "test_value".to_owned(), field_type: CronFieldType::DayOfMonth }), 1);
        match CronExpression::parse("* * * * *", &test_parser) {
            Err(CronParsingError::InvalidValue { value, field_type }) => { 
                assert_eq!("test_value", value);
                assert_eq!(CronFieldType::DayOfMonth, field_type);
            },
            _ => panic!("Did not get the expected error"),
        }
    }

    #[test]
    fn forwards_month_parse_error() {
        let test_parser = test_parser();
        test_parser.store_return_parse_month(|_| Err(CronParsingError::InvalidValue { value: "test_value".to_owned(), field_type: CronFieldType::Month }), 1);
        match CronExpression::parse("* * * * *", &test_parser) {
            Err(CronParsingError::InvalidValue { value, field_type }) => { 
                assert_eq!("test_value", value);
                assert_eq!(CronFieldType::Month, field_type);
            },
            _ => panic!("Did not get the expected error"),
        }
    }

    #[test]
    fn forwards_day_of_week_parse_error() {
        let test_parser = test_parser();
        test_parser.store_return_parse_day_of_week(|_| Err(CronParsingError::InvalidValue { value: "test_value".to_owned(), field_type: CronFieldType::DayOfWeek }), 1);
        match CronExpression::parse("* * * * *", &test_parser) {
            Err(CronParsingError::InvalidValue { value, field_type }) => { 
                assert_eq!("test_value", value);
                assert_eq!(CronFieldType::DayOfWeek, field_type);
            },
            _ => panic!("Did not get the expected error"),
        }
    }

    #[test]
    fn parses_each_field() {
        let test_parser = test_parser();
        test_parser.store_expected_input_parse_minute(|value| value == "one", 1);
        test_parser.store_expected_input_parse_hour(|value| value == "two", 1);
        test_parser.store_expected_input_parse_day_of_month(|value| value == "three", 1);
        test_parser.store_expected_input_parse_month(|value| value == "four", 1);
        test_parser.store_expected_input_parse_day_of_week(|value| value == "five", 1);

        CronExpression::parse("one two three four five", &test_parser).unwrap();
    }

    fn unresponsive_field() -> CronField { 
        CronField::new(CronFieldType::Minute, vec![CronValue::Single(99)]) 
    }
    fn responsive_field() -> CronField { 
        CronField::new(CronFieldType::Minute, vec![CronValue::Any])
    }

    #[test]
    fn any_unresponsive_field_is_unresponsive() {
        let expression = CronExpression {
            fields: [unresponsive_field(), responsive_field(), responsive_field(), responsive_field(), responsive_field()]
        };

        assert_eq!(false, expression.is_responsive(&Utc::now()));
    }

    #[test]
    fn all_responsive_fields_is_responsive() {
        let expression = CronExpression {
            fields: [responsive_field(), responsive_field(), responsive_field(), responsive_field(), responsive_field()]
        };

        assert_eq!(true, expression.is_responsive(&Utc::now()));
    }
}