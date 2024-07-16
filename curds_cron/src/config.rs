use super::*;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct CronConfig {
    pub jobs: Vec<JobConfig>,
}

#[derive(Serialize, Deserialize)]
pub struct JobConfig {
    pub name: String,
    pub expressions: Vec<String>,
    pub job: JobParameters,
}

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
    
    pub fn to_cron_jobs(self) -> Result<Vec<CronJob>, CronParsingError> {
        let mut jobs: Vec<CronJob> = Vec::new();
        for job in self.jobs {
            jobs.push(job.to_cron_job()?);
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

    pub fn to_cron_job(self) -> Result<CronJob, CronParsingError> {
        let mut expressions: Vec<CronExpression> = Vec::new();
        for expression in self.expressions {
            expressions.push(expression.parse()?);
        }

        Ok(CronJob::new(self.name, expressions, self.job))
    }
}

const SAMPLE_TIMEOUT: u64 = 50;
impl JobParameters {
    #[cfg(target_os = "windows")]
    pub fn sample() -> Self {
        Self {
            process: "cmd".to_owned(),
            arguments: Some(vec!["/C".to_owned(), "echo hello world!".to_owned()]),
            timeout_seconds: Some(SAMPLE_TIMEOUT),
        }
    }
    #[cfg(target_os = "linux")]
    pub fn sample() -> Self {
        Self {
            process: "sh".to_owned(),
            arguments: Some(vec!["-c".to_owned(), "echo hello world!".to_owned()]),
            timeout_seconds: Some(SAMPLE_TIMEOUT),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn absorb_is_expected() {
        let mut sample = CronConfig::sample();

        sample.absorb(CronConfig::sample());

        assert_eq!(2, sample.jobs.len());
    }
}