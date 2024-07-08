mod app;
mod config;
mod cron_job;
mod operations;

use config::*;
use operations::*;
use cron_job::*;
use curds_core::cli::cli_arguments;
use curds_core::cron::{DateTime, Timelike, TimeZone};
use curds_core::cron::{CronExpression, CronFieldParser, CronParsingError};
use curds_core::cli::*;
use app::CurdsCronApp;
use tokio::sync::oneshot::channel;
use tokio::time::sleep;
use tokio::signal::ctrl_c;
use async_process::Command;
use std::collections::HashSet;
use std::io::{stdout, Write};
use serde::{Serialize, Deserialize};

#[tokio::main]
async fn main() {
    let mut test_expressions: Vec<CronExpression> = Vec::new();
    let mut generate_paths: HashSet<Option<String>> = HashSet::new();
    let mut start_paths: Vec<Option<String>> = Vec::new();
    for operation in Cli::arguments::<CronOperation>() {
        match operation {
            CronOperation::Test(expressions) => test_expressions.extend(expressions),
            CronOperation::Generate { path } => { generate_paths.insert(path); },
            CronOperation::Start { path } => start_paths.push(path)
        }
    }

    let app = CurdsCronApp::new();
    app.test(test_expressions);
    app.generate(generate_paths).await;

    let (sender, receiver) = channel::<()>();
    tokio::spawn(async move {
        ctrl_c().await.expect("Failed to listen for ctrl+c");
        sender.send(()).expect("Failed to notify of closing");
    });

    tokio::select! {
        _ = app.start(start_paths) => {},
        _ = receiver => { println!("Closing down"); },
    };
}