use super::*;

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

#[derive(Debug, Serialize, Deserialize)]
pub struct JobParameters {
    pub process: String,
    pub arguments: Option<Vec<String>>,
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

        Ok(CronJob::new(self.name, expressions, self.job))
    }
}

impl JobParameters {
    pub fn sample() -> Self {
        Self {
            process: "cmd".to_owned(),
            arguments: Some(vec!["/C".to_owned(), "echo hello world!".to_owned()]),
        }
    }
}