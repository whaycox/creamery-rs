use super::*;

#[derive(Debug)]
pub struct CronJob {
    pub name: String,
    expressions: Vec<CronExpression>,
    pub parameters: JobParameters,
}

impl CronJob {
    pub fn new(name: String, expressions: Vec<CronExpression>, parameters: JobParameters) -> Self {
        Self {
            name,
            expressions,
            parameters,
        }
    }

    pub fn is_responsive<T: TimeZone>(&self, time: &DateTime<T>) -> bool {
        for expression in &self.expressions {
            if expression.is_responsive(time) {
                return true;
            }
        }
        false
    }
}