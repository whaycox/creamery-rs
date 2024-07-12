use super::*;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct CronConfig {
    jobs: Vec<JobConfig>,
}

#[derive(Serialize, Deserialize)]
pub struct JobConfig {
    name: String,
    expressions: Vec<String>,
    job: JobParameters,
}

pub const DEFAULT_TIMEOUT: u64 = 50;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JobParameters {
    pub process: String,
    pub arguments: Option<Vec<String>>,
    pub timeout_seconds: Option<u64>,
}

impl CronConfig {
    pub fn new() -> Self {
        Self {
            jobs: Vec::new(),
        }
    }

    pub fn sample() -> Self {
        Self {
            jobs: vec![JobConfig::sample()],
        }
    }

    pub fn absorb(&mut self, config: CronConfig) {
        self.jobs.extend(config.jobs);
    }
    
    pub fn parse<TParser: CronFieldParser>(self, parser: &TParser) -> Result<Vec<CronJob>, CronParsingError> {
        let mut jobs: Vec<CronJob> = Vec::new();
        for job in self.jobs {
            jobs.push(job.parse(parser)?);
        }

        Ok(jobs)
    }
}

impl JobConfig {
    pub fn sample() -> Self {
        Self {
            name: "Testing Job".to_owned(),
            expressions: vec!["* * * * *".to_owned()],
            job: JobParameters::sample(),
        }
    }

    pub fn parse<TParser: CronFieldParser>(self, parser: &TParser) -> Result<CronJob, CronParsingError> {
        let mut expressions: Vec<CronExpression> = Vec::new();
        for expression in self.expressions {
            expressions.push(CronExpression::parse(&expression, parser)?);
        }

        Ok(CronJob::new(self.name, expressions, self.job.expand()))
    }
}

impl JobParameters {
    pub fn sample() -> Self {
        Self {
            process: "cmd".to_owned(),
            arguments: Some(vec!["/C".to_owned(), "echo hello world!".to_owned()]),
            timeout_seconds: Some(DEFAULT_TIMEOUT),
        }
    }

    pub fn expand(self) -> Self {
        let expanded_timeout = if self.timeout_seconds.is_some() {
            self.timeout_seconds
        }
        else {
            Some(DEFAULT_TIMEOUT)
        };

        Self {
            process: self.process,
            arguments: self.arguments,
            timeout_seconds: expanded_timeout,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{CronConfig, JobParameters, DEFAULT_TIMEOUT};

    #[test]
    fn absorb_is_expected() {
        let mut sample = CronConfig::sample();

        sample.absorb(CronConfig::sample());

        assert_eq!(2, sample.jobs.len());
    }

    #[test]
    fn expand_populates_timeout() {
        let mut test = JobParameters::sample();
        test.timeout_seconds = None;

        test = test.expand();

        assert_eq!(test.timeout_seconds, Some(DEFAULT_TIMEOUT));
    }
}