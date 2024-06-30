use super::*;

pub struct CronValueParserLink {
    handler: CronParsingHandler,
    successor: Option<Box<CronValueParserLink>>,
}
impl CronValueParserLink {
    pub fn tail(handler: CronParsingHandler) -> CronValueParserLink {
        CronValueParserLink {
            handler: handler,
            successor: None,
        }
    }

    pub fn prepend(self, handler: CronParsingHandler) -> CronValueParserLink {
        CronValueParserLink {
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
                None => Err(CronParsingError::InvalidValue(value.to_owned())),
                Some(link) => return link.parse(field_type, value),
            }
        }
    }
}