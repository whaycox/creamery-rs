use super::*;

#[derive(Debug)]
pub struct CronExpression {
    fields: [CronField; Self::FIELD_COUNT],
}

impl CronExpression {
    const FIELD_COUNT : usize = 5;
    const FIELD_DELIMITER : char = ' ';

    pub fn test(&self) -> bool {
        true
    }

    pub fn parse<TParser>(expression: &str) -> Result<CronExpression, CronParsingError>
    where TParser : CronFieldParser {
        let parts: Vec<&str> = expression
            .split(Self::FIELD_DELIMITER)
            .filter(|part| part.len() > 0)
            .collect();
        if parts.len() != Self::FIELD_COUNT {
            return Err(CronParsingError::FieldCount(expression.to_owned(), parts.len()));
        }
        Ok(CronExpression { 
            fields : [   
                TParser::parse_minute(parts[0])?,
                TParser::parse_hour(parts[1])?,
                TParser::parse_day_of_month(parts[2])?,
                TParser::parse_month(parts[3])?,
                TParser::parse_day_of_week(parts[4])?,
            ]
        })
    }

}