use super::*;
use std::{collections::HashSet, time::Duration};
use curds_core::{cron::CurdsCronFieldParser, io::{AsyncFileSystem, FileSystem}, time::*};
use uuid::Uuid;

const DEFAULT_CONFIG: &str = "config.json";
const SLEEP_TIME_S: u64 = 7;
pub struct CurdsCronApp<
TClock : Clock,
TFileSystem : FileSystem,
TParser : CronFieldParser,
TProcessor : Processor> {
    clock: TClock,
    file_system: TFileSystem,
    parser: TParser,
    processor: TProcessor,
}

impl CurdsCronApp<MachineClock, AsyncFileSystem, CurdsCronFieldParser, JobProcessor> {
    pub fn new() -> Self {
        Self {
            clock: MachineClock,
            file_system: AsyncFileSystem,
            parser: CurdsCronFieldParser,
            processor: JobProcessor,
        }
    }
}

impl<TClock, TFileSystem, TParser, TProcessor> CurdsCronApp<TClock, TFileSystem, TParser, TProcessor> where
TClock : Clock,
TFileSystem : FileSystem,
TParser : CronFieldParser,
TProcessor : Processor {
    pub fn test(&self, expressions: Vec<CronExpression>) {
        if expressions.len() > 0 {
            log::info!("Beginning a test of {} provided expressions", expressions.len());

            let current = self.clock.current();
            for expression in &expressions {
                log::info!("Testing {} - {:#?}", expression, expression);
                if expression.is_responsive(&current) {
                    log::info!("{} is responsive at {}", expression, current);
                }
                else {
                    log::info!("{} is not responsive at {}", expression, current);
                }
            }
        }
    }

    fn expand_path(path: &Option<String>) -> &str {
        match &path {
            Some(provided_path) => provided_path,
            None => DEFAULT_CONFIG,
        }  
    }

    pub async fn generate(&self, paths: HashSet<Option<String>>) {
        if paths.len() > 0 {
            log::info!("Generating {} sample configs", paths.len());
            for path in paths {
                let expanded_path = Self::expand_path(&path);
                let json_data = serde_json::to_string_pretty(&CronConfig::sample()).unwrap();
                self.file_system.write_bytes(&expanded_path, json_data.as_bytes()).await.unwrap();
            }
        }
    }

    pub async fn start(&self, paths: HashSet<Option<String>>) {
        if paths.len() > 0 {
            log::info!("Starting from {} configurations", paths.len());
            let mut combined = CronConfig::new();
            for path in paths {
                let expanded_path = Self::expand_path(&path);
                let file = self.file_system.read_string(&expanded_path).await.unwrap();
                let config: CronConfig = serde_json::from_str(&file).unwrap();
                combined.absorb(config);
            }
            let jobs = combined.parse(&self.parser).unwrap();
            log::info!("Configured jobs: {:#?}", jobs);

            let mut last_minute = None;
            loop {
                let current = self.clock.current();
                let current_minute = Some(current.minute());
                if current_minute != last_minute {
                    last_minute = current_minute;
    
                    for job in &jobs {
                        if job.is_responsive(&current) {
                            let invoke_id = Uuid::new_v4();
                            log::info!("{}|{} is responsive; invoking", invoke_id, job.name);
                            tokio::spawn(self.processor.process_job(invoke_id, job.parameters.clone()));
                        }
                    }
                }

                sleep(Duration::from_secs(SLEEP_TIME_S)).await;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::OnceLock;
    use curds_core::{cron::TestingCronFieldParser, io::TestingFileSystem, time::DateTime};

    use super::*;

    static TESTING_TIME: OnceLock<DateTime<Local>> = OnceLock::new();
    // impl CurdsCronApp<TestingClock, TestingFileSystem, TestingCronFieldParser, TestingProcessor> {
    //     pub fn test_object() -> Self {
    //         let test_object = Self {
    //             clock: TestingClock::new(),
    //             file_system: TestingFileSystem::new(),
    //             parser: TestingCronFieldParser::new(),
    //             processor: TestingProcessor::new(),
    //         };
    //         test_object.clock.default_return_current(|| TESTING_TIME.get_or_init(|| Local::now()).clone());

    //         test_object
    //     }
    // }
    
    // #[tokio::test]
    // async fn calls_local_time() {
    //     let test_object = CurdsCronApp::test_object();
    //     test_object.clock.expect_calls_current(1);


    //     todo!("local time test");
    // }
}