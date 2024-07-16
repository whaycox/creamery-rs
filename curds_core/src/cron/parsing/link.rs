use super::*;

type CronParsingHandler = fn(&str, &CronFieldType) -> Option<Result<CronValue, CronParsingError>>;

pub struct CronValueParserLink {
    handler: CronParsingHandler,
    successor: Option<Box<CronValueParserLink>>,
}
impl CronValueParserLink {
    pub fn tail(handler: CronParsingHandler) -> Self {
        Self {
            handler: handler,
            successor: None,
        }
    }

    pub fn prepend(self, handler: CronParsingHandler) -> Self {
        Self {
            handler: handler,
            successor: Some(Box::new(self)),
        }
    }

    pub fn parse(&self, field_type: &CronFieldType, value: &str) -> Result<CronValue, CronParsingError> {
        if let Some(cron_value) = (self.handler)(value, field_type) {
            return Ok(cron_value?);
        }
        else {        
            match &self.successor {
                None => Err(CronParsingError::InvalidValue {
                    value: value.to_owned(),
                    field_type: field_type.clone(),
                }),
                Some(link) => return link.parse(field_type, value),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_VALUE: &str = "TestValue";

    fn none_handler(_: &str, _: &CronFieldType) -> Option<Result<CronValue, CronParsingError>> { None }
    fn some_handler(_: &str, _: &CronFieldType) -> Option<Result<CronValue, CronParsingError>> { Some(Ok(CronValue::Any)) }

    #[test]
    fn tail_sets_no_successor() {
        let test_object = CronValueParserLink::tail(none_handler);

        assert_eq!(true, test_object.successor.is_none());
    }

    #[test]
    fn prepend_makes_self_successor() {
        let tail = CronValueParserLink::tail(some_handler);
        let test_object = tail.prepend(none_handler);

        assert_eq!(true, test_object.successor.is_some());
    }

    #[test]
    fn parse_returns_current_value_if_some() {
        let test_object = CronValueParserLink::tail(some_handler);

        assert_eq!(Ok(CronValue::Any), test_object.parse(&CronFieldType::Minute, TEST_VALUE));
    }

    #[test]
    fn parse_chains_to_successor() {
        let test_object = CronValueParserLink::tail(some_handler)
            .prepend(none_handler);

        assert_eq!(Ok(CronValue::Any), test_object.parse(&CronFieldType::Minute, TEST_VALUE));
    }

    #[test]
    fn parse_returns_error_if_no_succesor() {
        let test_object = CronValueParserLink::tail(none_handler);

        match test_object.parse(&CronFieldType::Minute, TEST_VALUE) {
            Err(CronParsingError::InvalidValue { value, field_type }) => {
                assert_eq!(TEST_VALUE, value);
                assert_eq!(CronFieldType::Minute, field_type);
            },
            _ => panic!("Did not get the expected error"),
        }
    }
}