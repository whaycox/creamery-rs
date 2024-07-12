use super::*;
use std::time::Duration;
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

    pub async fn generate(&self, paths: Vec<Option<String>>) {
        if paths.len() > 0 {
            log::info!("Generating {} sample configs", paths.len());
            for path in paths {
                let expanded_path = Self::expand_path(&path);
                let json_data = serde_json::to_string_pretty(&CronConfig::sample()).unwrap();
                self.file_system.write_bytes(&expanded_path, json_data.as_bytes()).await.unwrap();
            }
        }
    }

    pub async fn start(&self, paths: Vec<Option<String>>) {
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
    impl CurdsCronApp<TestingClock, TestingFileSystem, TestingCronFieldParser, TestingProcessor> {
        pub fn test_object() -> Self {
            let test_object = Self {
                clock: TestingClock::new(),
                file_system: TestingFileSystem::new(),
                parser: TestingCronFieldParser::new(),
                processor: TestingProcessor::new(),
            };
            test_object.clock.default_return_current(|| TESTING_TIME.get_or_init(|| Local::now()).clone());
            test_object.file_system.default_return_read_string(|_| Ok(sample_json()));
            test_object.parser.default_return_parse_minute(|_| Ok(CronField::new(CronFieldType::Minute, vec![CronValue::Any])));
            test_object.parser.default_return_parse_hour(|_| Ok(CronField::new(CronFieldType::Hour, vec![CronValue::Any])));
            test_object.parser.default_return_parse_day_of_month(|_| Ok(CronField::new(CronFieldType::DayOfMonth, vec![CronValue::Any])));
            test_object.parser.default_return_parse_month(|_| Ok(CronField::new(CronFieldType::Month, vec![CronValue::Any])));
            test_object.parser.default_return_parse_day_of_week(|_| Ok(CronField::new(CronFieldType::DayOfWeek, vec![CronValue::Any])));
            test_object.processor.default_return_process_job(|_,_| Box::pin(async {}));

            test_object
        }
    }

    fn test_expressions() -> Vec<CronExpression> {
        vec![std::str::FromStr::from_str("* * * * *").unwrap()]
    }

    const TEST_PATH: &str = "TestPath";
    fn test_paths() -> Vec<Option<String>> {
        vec![None, Some(TEST_PATH.to_owned())]
    }

    fn sample_json() -> String {
        serde_json::to_string_pretty(&CronConfig::sample()).unwrap()
    }
    
    #[test]
    fn test_calls_local_time() {
        let test_object = CurdsCronApp::test_object();
        test_object.clock.expect_calls_current(1);

        test_object.test(test_expressions());
    }

    #[tokio::test]
    async fn generate_writes_expected_files() {
        let test_object = CurdsCronApp::test_object();
        test_object.file_system.default_return_write_bytes(|_,_| Ok(()));
        test_object.file_system.store_expected_input_write_bytes(|path, bytes| path == DEFAULT_CONFIG && bytes == sample_json().as_bytes(), 1);
        test_object.file_system.store_expected_input_write_bytes(|path, bytes| path == TEST_PATH && bytes == sample_json().as_bytes(), 1);

        test_object.generate(test_paths()).await;
    }

    #[tokio::test]
    async fn start_reads_paths() {
        let test_object = CurdsCronApp::test_object();
        test_object.file_system.store_expected_input_read_string(|path| path == DEFAULT_CONFIG, 1);
        test_object.file_system.store_expected_input_read_string(|path| path == TEST_PATH, 1);

        tokio::time::timeout(Duration::from_millis(100), test_object.start(test_paths())).await.expect_err("");
    }

    #[tokio::test]
    async fn start_parses_expression() {
        let test_object = CurdsCronApp::test_object();
        test_object.parser.store_expected_input_parse_minute(|value| value == "*", 1);
        test_object.parser.store_expected_input_parse_hour(|value| value == "*", 1);
        test_object.parser.store_expected_input_parse_day_of_month(|value| value == "*", 1);
        test_object.parser.store_expected_input_parse_month(|value| value == "*", 1);
        test_object.parser.store_expected_input_parse_day_of_week(|value| value == "*", 1);

        tokio::time::timeout(Duration::from_millis(100), test_object.start(test_paths())).await.expect_err("");
    }

    #[tokio::test]
    async fn start_calls_local() {
        let test_object = CurdsCronApp::test_object();
        test_object.clock.expect_calls_current(1);

        tokio::time::timeout(Duration::from_millis(100), test_object.start(test_paths())).await.expect_err("");
    }

    #[tokio::test]
    async fn start_processes_job() {
        let test_object = CurdsCronApp::test_object();
        test_object.processor.expect_calls_process_job(2);

        tokio::time::timeout(Duration::from_millis(100), test_object.start(test_paths())).await.expect_err("");
    }

    #[tokio::test]
    async fn start_doesnt_process_if_expression_isnt_responsive() {
        let test_object = CurdsCronApp::test_object();
        test_object.parser.default_return_parse_minute(|_| Ok(CronField::new(CronFieldType::Minute, vec![])));
        test_object.processor.expect_calls_process_job(0);

        tokio::time::timeout(Duration::from_millis(100), test_object.start(test_paths())).await.expect_err("");
    }
}