use super::*;

#[derive(Debug)]
pub struct CronExpression {
    fields: [CronField; Self::FIELD_COUNT],
}

impl FromStr for CronExpression {
    type Err = CronParsingError;

    fn from_str(string: &str) -> Result<Self, Self::Err> { 
        CronExpression::parse::<CurdsCronParser>(string)
    }
}

impl CronExpression {
    const FIELD_COUNT : usize = 5;

    fn parse<TFieldParser>(expression: &str) -> Result<CronExpression, CronParsingError>
    where TFieldParser : CronFieldParser {
        let parts: Vec<&str> = expression.split(" ").collect();
        if parts.iter().any(|part| { part.len() == 0}) {
            return Err(CronParsingError::EmptyField {
                expression: expression.clone().to_owned(),
            });
        }
        if parts.len() != Self::FIELD_COUNT {
            return Err(CronParsingError::FieldCount { 
                expression: expression.clone().to_owned(),
                parts: parts.len()
            });
        }
        Ok(CronExpression { 
            fields : [   
                TFieldParser::parse(CronDatePart::Minutes, parts[0])?,
                TFieldParser::parse(CronDatePart::Hours, parts[1])?,
                TFieldParser::parse(CronDatePart::DayOfMonth, parts[2])?,
                TFieldParser::parse(CronDatePart::Month, parts[3])?,
                TFieldParser::parse(CronDatePart::DayOfWeek, parts[4])?,
            ]
        })
    }

    pub fn is_match<Tz>(&self, datetime: &DateTime<Tz>) -> bool 
    where Tz: TimeZone {
        for field in &self.fields {
            if !field.is_match(datetime) {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn too_long_expression_fails() {
        CronExpression::parse::<CurdsCronParser>("* * * * * *").expect_err("Expected a long expression to fail");
    }

    #[test]
    fn too_short_expression_fails() {
        CronExpression::parse::<CurdsCronParser>("* * * *").expect_err("Expected a short expression to fail");
    }

    macro_rules! expect_parsing {
        ($context:expr => ($($expected_part:expr, $expected_value:expr),+)) => {
            $(
                $context
                    .expect()
                    .with(predicate::eq($expected_part), predicate::eq($expected_value))
                    .times(1)
                    .returning(|_, _| {
                        Ok(CronField::new($expected_part, Vec::<CronValue>::new()))
                    });
            )+
        };
    }

    #[test]
    fn parses_correctly_with_parser() -> Result<(), CronParsingError> {
        let context = MockCronFieldParser::parse_context();
        expect_parsing! { context =>
            (
                CronDatePart::Minutes, "Minutes",
                CronDatePart::Hours, "Hours",
                CronDatePart::DayOfMonth, "DayOfMonth",
                CronDatePart::Month, "Month",
                CronDatePart::DayOfWeek, "DayOfWeek"
            )
        }

        CronExpression::parse::<MockCronFieldParser>("Minutes Hours DayOfMonth Month DayOfWeek")?;
        Ok(())
    }

    fn false_field() -> CronField { 
        CronField::new(CronDatePart::Minutes, vec![CronValue::Single(99)]) 
    }
    fn true_field() -> CronField { 
        CronField::new(CronDatePart::Minutes, vec![CronValue::Any])
    }

    #[test]
    fn any_nonmatch_field_returns_false() {
        let expression = CronExpression {
            fields: [true_field(), false_field(), true_field(), true_field(), true_field()]
        };

        assert_eq!(false, expression.is_match(&Utc::now()));
    }

    #[test]
    fn all_fields_match_returns_true() {
        let expression = CronExpression {
            fields: [true_field(), true_field(), true_field(), true_field(), true_field()]
        };

        assert_eq!(true, expression.is_match(&Utc::now()));
    }
}