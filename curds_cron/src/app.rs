use super::*;
use std::{collections::HashSet, time::Duration};
use curds_core::{cron::CurdsCronFieldParser, io::{AsyncFileSystem, FileSystem}, time::*};
use tokio::io::{AsyncWriteExt, AsyncReadExt};

const DEFAULT_CONFIG: &str = "config.json";
const SLEEP_TIME_S: u64 = 7;
pub struct CurdsCronApp<
TClock : Clock,
TFileSystem : FileSystem,
TParser : CronFieldParser> {
    clock: TClock,
    file_system: TFileSystem,
    parser: TParser,
}

impl CurdsCronApp<MachineClock, AsyncFileSystem, CurdsCronFieldParser> {
    pub fn new() -> Self {
        Self {
            clock: MachineClock,
            file_system: AsyncFileSystem,
            parser: CurdsCronFieldParser,
        }
    }
}

impl<TClock, TFileSystem, TParser> CurdsCronApp<TClock, TFileSystem, TParser> where
TClock : Clock,
TFileSystem : FileSystem,
TParser : CronFieldParser {
    pub fn test(&self, expressions: Vec<CronExpression>) {
        if expressions.len() > 0 {
            println!("Beginning a test of {} provided expressions", expressions.len());

            let current = self.clock.current();
            println!("Testing at {}", current);
            for expression in &expressions {
                if expression.is_responsive(&current) {
                    println!("{} is responsive", expression);
                }
                else {
                    println!("{} is not responsive", expression);
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
            println!("Generating {} sample configs", paths.len());
            for path in paths {
                let expanded_path = Self::expand_path(&path);
                println!("Generating: {}", expanded_path);

                let json_data = serde_json::to_string_pretty(&CronConfig::sample()).unwrap();
                let mut file = self.file_system.write(&expanded_path).await.unwrap();
                file.write_all(json_data.as_bytes()).await.unwrap();
            }
        }
    }

    pub async fn start(&self, paths: Vec<Option<String>>) {
        if paths.len() > 0 {
            println!("Starting from {} configurations", paths.len());
            let mut combined = CronConfig::new();
            for path in paths {
                let expanded_path = Self::expand_path(&path);
                println!("Reading: {}", expanded_path);

                let mut file = self.file_system.read(&expanded_path).await.unwrap();
                let mut contents = String::new();
                file.read_to_string(&mut contents).await.unwrap();

                let config: CronConfig = serde_json::from_str(&contents).unwrap();
                combined.absorb(config);
            }
            let jobs = combined.parse(&self.parser).unwrap();
            println!("Configured jobs: {:#?}", jobs);

            let mut last_minute = None;
            loop {
                let current = self.clock.current();
                let current_minute = Some(current.minute());
                if current_minute != last_minute {
                    last_minute = current_minute;
    
                    println!("Checking at {}", current);
                    for job in &jobs {
                        if job.is_responsive(&current) {
                            println!("{} is responsive; invoking its parameters", job.name);
                            let mut command = Command::new(&job.parameters.process);
                            if let Some(parameters) = &job.parameters.arguments {
                                command.args(parameters);
                            }

                            match command.output().await {
                                Ok(result) => {
                                    println!("It exited with {}", result.status);
                                    println!("Its output:");
                                    stdout().write_all(&result.stdout).unwrap();
                                },
                                Err(error) => println!("It failed to run: {}", error),
                            }
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
    impl CurdsCronApp<TestingClock, TestingFileSystem, TestingCronFieldParser> {
        pub fn test_object() -> Self {
            let test_object = Self {
                clock: TestingClock::new(),
                file_system: TestingFileSystem::new(),
                parser: TestingCronFieldParser::new(),
            };
            test_object.clock.default_return_current(|| TESTING_TIME.get_or_init(|| Local::now()).clone());

            test_object
        }
    }
    
    #[tokio::test]
    async fn calls_local_time() {
        let test_object = CurdsCronApp::test_object();
        test_object.clock.expect_calls_current(1);


        todo!("local time test");
    }
}