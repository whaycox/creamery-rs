mod app;
mod config;
mod cron_job;
mod operations;
mod processor;

use config::*;
use operations::*;
use cron_job::*;
use processor::*;

use curds_core::cron::*;
use curds_core::cli::*;

use std::pin::Pin;
use std::future::Future;
use std::time::Duration;
use std::collections::HashSet;
use std::process::Stdio;
use tokio::sync::oneshot::channel;
use tokio::time::sleep;
use tokio::signal::ctrl_c;
use tokio::process::{Command, Child};
use tokio::io::BufReader;
use tokio::io::AsyncBufReadExt;
use uuid::Uuid;

#[tokio::main]
async fn main() {
    curds_core::logger::initialize();
    let parsed_arguments = Cli::arguments::<CronOperation>();
    if parsed_arguments.len() == 0 {
        log::error!("No arguments were provided");
        Cli::usage::<CronOperation>();
    }
    else {
        let mut test_expressions: Vec<CronExpression> = Vec::new();
        let mut generate_paths: HashSet<Option<String>> = HashSet::new();
        let mut start_paths: HashSet<Option<String>> = HashSet::new();
        for operation in parsed_arguments {
            match operation {
                CronOperation::Test(expressions) => test_expressions.extend(expressions),
                CronOperation::Generate { path } => { generate_paths.insert(path); },
                CronOperation::Start { path } => { start_paths.insert(path); }, 
            }
        }
    
        let app = app::CurdsCronApp::new();
        app.test(test_expressions);
        app.generate(generate_paths).await;
    
        let (sender, receiver) = channel::<()>();
        tokio::spawn(async move {
            ctrl_c().await.expect("Failed to listen for ctrl+c");
            sender.send(()).expect("Failed to notify of closing");
        });
    
        tokio::select! {
            _ = app.start(start_paths) => {},
            _ = receiver => { log::info!("Closing down"); },
        };
    }
}