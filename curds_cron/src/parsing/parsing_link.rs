use super::*;

pub struct CronValueParserLink {
    pub current: Box<dyn CronValueParsingHandler>,
    pub successor: Option<Box<CronValueParserLink>>,
}
impl CronValueParserLink {
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

    pub fn parse(&self, date_part: &CronDatePart, value: &str) -> Result<CronValue, CronParsingError> {
        if let Some(cron_value) = self.current.parse(date_part, value) {
            return Ok(cron_value?);
        }
        else {        
            match &self.successor {
                None => Err(CronParsingError::InvalidValue {
                    date_part: *date_part,
                    value: value.to_owned(),
                 }),
                Some(link) => return link.parse(date_part, value),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_VALUE: &str = "TestValue";

    #[test]
    fn tail_sets_no_successor() {
        let handler = Box::<MockCronValueParsingHandler>::new(MockCronValueParsingHandler::new());
        let test_object = CronValueParserLink::tail(handler);

        if let Some(_successor) = test_object.successor {
            panic!("Successor should be None with tail()");
        }
    }

    #[test]
    fn prepend_makes_self_successor() {
        let tail = CronValueParserLink::tail(Box::<MockCronValueParsingHandler>::new(MockCronValueParsingHandler::new()));
        let test_object = tail.prepend(Box::<MockCronValueParsingHandler>::new(MockCronValueParsingHandler::new()));

        if let None = test_object.successor {
            panic!("Successor should be Some() with prepend()");
        }
    }

    #[test]
    fn parse_returns_current_value_if_some() {
        let mut mock_handler = MockCronValueParsingHandler::new();
        mock_handler.expect_parse()
            .with(predicate::eq(CronDatePart::Minutes), predicate::eq(TEST_VALUE))
            .times(1)
            .returning(|_, _| { Some(Ok(CronValue::Any)) });
        let test_object = CronValueParserLink::tail(Box::<MockCronValueParsingHandler>::new(mock_handler));

        let actual = test_object.parse(&CronDatePart::Minutes, TEST_VALUE);

        assert_eq!(Ok(CronValue::Any), actual);
    }

    #[test]
    fn parse_chains_to_successor() {
        let mut sequence = Sequence::new();
        let mut mock_first_handler = MockCronValueParsingHandler::new();
        mock_first_handler.expect_parse()
            .with(predicate::eq(CronDatePart::Minutes), predicate::eq(TEST_VALUE))
            .times(1)
            .in_sequence(&mut sequence)
            .returning(|_, _| { None });
        let mut mock_second_handler = MockCronValueParsingHandler::new();
        mock_second_handler.expect_parse()
            .with(predicate::eq(CronDatePart::Minutes), predicate::eq(TEST_VALUE))
            .times(1)
            .in_sequence(&mut sequence)
            .returning(|_, _| { Some(Ok(CronValue::Any)) });
        let test_object = CronValueParserLink::tail(Box::<MockCronValueParsingHandler>::new(mock_second_handler))
            .prepend(Box::<MockCronValueParsingHandler>::new(mock_first_handler));

        let actual = test_object.parse(&CronDatePart::Minutes, TEST_VALUE);

        assert_eq!(Ok(CronValue::Any), actual);
    }

    #[test]
    fn parse_returns_error_if_no_succesor() {
        let mut mock_handler = MockCronValueParsingHandler::new();
        mock_handler.expect_parse()
            .with(predicate::eq(CronDatePart::Minutes), predicate::eq(TEST_VALUE))
            .times(1)
            .returning(|_, _| { None });
        let test_object = CronValueParserLink::tail(Box::<MockCronValueParsingHandler>::new(mock_handler));

        test_object.parse(&CronDatePart::Minutes, TEST_VALUE).expect_err("Expected parsing to error but returned");
    }
}