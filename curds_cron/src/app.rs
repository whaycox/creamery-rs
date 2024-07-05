use curds_core::{cron::CurdsCronFieldParser, time::*};

use super::*;
use std::{marker::PhantomData, time::Duration};

const SLEEP_TIME_S: u64 = 7;
pub struct CurdsCronApp<
TFactory: ArgumentFactory,
TClock: Clock, 
TParser: CronFieldParser> {
    arguments: TFactory,
    clock: TClock,
    parser: TParser,
}

impl CurdsCronApp<CliArgumentFactory, MachineClock, CurdsCronFieldParser> {
    pub fn new() -> Self {
        Self {
            arguments: CliArgumentFactory,
            clock: MachineClock,
            parser: CurdsCronFieldParser,
        }
    }
}

impl<TFactory, TClock, TParser> CurdsCronApp<TFactory, TClock, TParser> where 
TFactory : ArgumentFactory,
TClock: Clock,
TParser : CronFieldParser {
    pub async fn start(&self) {
        println!("Starting");
        let mut expressions: Vec<CronExpression> = Vec::new();
        for arg in self.arguments.create() {
            match CronExpression::parse(&arg, &self.parser) {
                Ok(expression) => expressions.push(expression),
                Err(error) => println!("\"{}\" is not a valid cron expression: {}", arg, error),
            }
        }
        println!("Supplied {} expressions", expressions.len());

        let mut last_minute = None;
        loop {
            let current = self.clock.current();
            let current_minute = Some(current.minute());
            if current_minute != last_minute {
                last_minute = current_minute;

                println!("Testing for {}", current);
                for expression in &expressions {
                    if expression.is_responsive(&current) {
                        println!("{} is responsive, running test", expression);
                        let result = Command::new("cmd")
                            .args(["/C", "echo hello"])
                            .output()
                            .await
                            .expect("failed to execute process");
                        
                        println!("It exited with a status code: {}", result.status);
                        println!("Its output:");
                        stdout().write_all(&result.stdout).unwrap();
                    }
                    else {
                        println!("{} is not responsive", expression);
                    }
                }
            }

            sleep(Duration::from_secs(SLEEP_TIME_S)).await;
        }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::OnceLock;
    use curds_core::time::DateTime;

    use super::*;

    static TESTING_TIME: OnceLock<DateTime<Local>> = OnceLock::new();
    impl CurdsCronApp<TestingArgumentFactory, TestingClock, CurdsCronFieldParser> {
        pub fn test_object() -> Self {
            let test_object = Self {
                arguments: TestingArgumentFactory::new(),
                clock: TestingClock::new(),
                parser: CurdsCronFieldParser,
            };
            test_object.arguments.default_return_create(|| Vec::new());
            test_object.clock.default_return_current(|| TESTING_TIME.get_or_init(|| Local::now()).clone());

            test_object
        }
    }
    
    #[tokio::test]
    async fn calls_local_time() {
        let test_object = CurdsCronApp::test_object();
        test_object.clock.expect_calls_current(1);

        tokio::select! {
            _ = test_object.start() => {},
            _ = sleep(Duration::from_millis(100)) => {},
        }
    }
}