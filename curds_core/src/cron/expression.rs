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