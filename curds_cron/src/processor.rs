use std::time::Duration;
use tokio::process::{Child, ChildStdout};
use tokio::io::{BufReader, Lines};

use super::*;
use curds_core::whey_mock;

#[whey_mock]
pub trait Processor {
    fn process_job(&self, id: Uuid, job: JobParameters) -> Pin<Box<dyn Future<Output = ()> + Send>>;
}

pub struct JobProcessor;

impl Processor for JobProcessor {
    fn process_job(&self, id: Uuid, job: JobParameters) -> Pin<Box<dyn Future<Output = ()> + Send>> {
        Box::pin(async move {
            let mut command = Command::new(&job.process);
            command.stdout(Stdio::piped());
            if let Some(parameters) = &job.arguments {
                command.args(parameters);
            }
        
            let mut child = command.spawn().unwrap();
            let reader = BufReader::new(child.stdout.take().unwrap()).lines();
            if let Some(timeout) = job.timeout_seconds {
                process_with_timeout(id, child, reader, timeout).await;
            }
            else {
                process_unbounded(id, child, reader).await;
            }
        })
    }
}

async fn process_with_timeout(id: Uuid, mut child: Child, mut reader: Lines<BufReader<ChildStdout>>, timeout: u64) {
    match tokio::time::timeout(Duration::from_secs(timeout), child.wait()).await {
        Ok(result) => match result {
            Ok(exit_status) => {
                log::info!("{}|It exited with {}", id, exit_status);
                while let Some(line) = reader.next_line().await.unwrap() {
                    log::info!("{}|{}", id, line);
                }
            },
            Err(failure) => {
                log::error!("{}|Failed to run: {}", id, failure);
            },
        },
        Err(_) => { 
            log::error!("{}|It timed out after {} seconds; killing process", id, timeout);
            
            #[cfg(target_os = "windows")]
            {
                let pid = child.id().unwrap();
                match Command::new("taskkill")
                    .args(&["/f", "/t", "/pid", &pid.to_string()])
                    .stdout(Stdio::null())
                    .stderr(Stdio::null())
                    .status()
                    .await {
                        Ok(_) => {},
                        Err(err) => {
                            log::error!("{}|Failed to kill process: {}", id, err);
                        },
                    }
            }
            #[cfg(target_os = "linux")]
            {
                let pid = child.id().unwrap();
                match Command::new("pkill")
                    .args(&["-P", &pid.to_string()])
                    .stdout(Stdio::null())
                    .stderr(Stdio::null())
                    .status()
                    .await {
                        Ok(_) => {},
                        Err(err) => {
                            log::error!("{}|Failed to kill process: {}", id, err);
                        },
                    }
            }
        }
    }
}

async fn process_unbounded(id: Uuid, mut child: Child, mut reader: Lines<BufReader<ChildStdout>>) {
    match child.wait().await {
        Ok(exit_status) => {
            log::info!("{}|It exited with {}", id, exit_status);
            while let Some(line) = reader.next_line().await.unwrap() {
                log::info!("{}|{}", id, line);
            }
        },
        Err(failure) => {
            log::error!("{}|Failed to run: {}", id, failure);
        },
    }
}