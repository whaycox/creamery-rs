use super::*;

pub struct CronExpression {
    fields: Vec<CronField>,
}

impl CronExpression {
    pub fn is_match<Tz>(&self, datetime: &DateTime<Tz>) -> bool 
    where Tz: TimeZone {
        for field in &self.fields {
            if !field.is_match(datetime) {
                return false;
            }
        }
        true
    }

    pub fn parse(expression: &str) -> CronExpression { 
        CronExpression::parse_with_parser::<CurdsCronParser>(expression) 
    }

    fn parse_with_parser<TFieldParser>(expression: &str) -> CronExpression
    where TFieldParser : CronFieldParser {
        const EXPRESSION_FIELD_COUNT: usize = 5;
        let parts: Vec<&str> = expression.split(" ").collect();
        if parts.len() != EXPRESSION_FIELD_COUNT {
            panic!("Invalid expression supplied: {}", expression);
        }
        let fields = vec![
            TFieldParser::parse(CronDatePart::Minutes, parts[0]),
            TFieldParser::parse(CronDatePart::Hours, parts[1]),
            TFieldParser::parse(CronDatePart::DayOfMonth, parts[2]),
            TFieldParser::parse(CronDatePart::Month, parts[3]),
            TFieldParser::parse(CronDatePart::DayOfWeek, parts[4]),
        ];
        CronExpression { fields }
    }
}

#[cfg(test)]
mod tests {
}