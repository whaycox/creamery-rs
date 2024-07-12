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

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;
    use curds_core::time::Local;

    fn responsive() -> CronExpression {
        FromStr::from_str("* * * * *").unwrap()
    }

    fn unresponsive() -> CronExpression {
        FromStr::from_str("0 0 1 1 0").unwrap()
    }

    #[test]
    fn only_unresponsive_is_unresponsive() {
        let test = CronJob {
            name: "Testing".to_owned(),
            expressions: vec![unresponsive()],
            parameters: JobParameters::sample(),
        };

        assert_eq!(false, test.is_responsive(&Local::now()));
    }

    #[test]
    fn any_responsive_is_responsive() {
        let test = CronJob {
            name: "Testing".to_owned(),
            expressions: vec![responsive(), unresponsive()],
            parameters: JobParameters::sample(),
        };

        assert_eq!(true, test.is_responsive(&Local::now()));
    }
}