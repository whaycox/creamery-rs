mod app;
mod scheduled_job;
mod operations;

use curds_core::cli::cli_arguments;
use curds_core::cron::Timelike;
use curds_core::cron::{CronExpression, CronFieldParser};
use curds_core::cli::*;
use app::CurdsCronApp;
use tokio::sync::oneshot::channel;
use tokio::time::sleep;
use tokio::signal::ctrl_c;
use async_process::Command;
use std::io::{stdout, Write};

#[tokio::main]
async fn main() {
    let (sender, receiver) = channel::<()>();

    tokio::spawn(async move {
        ctrl_c().await.expect("Failed to listen for ctrl+c");
        sender.send(()).expect("Failed to notify of closing");
    });

    let app = CurdsCronApp::new(); 
    tokio::select! {
        _ = app.start() => {},
        _ = receiver => { println!("Closing down"); },
    };
}