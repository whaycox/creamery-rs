mod app;
mod argument_factory;

use curds_core::cron::{CronExpression, CronFieldParser};
use app::CurdsCronApp;
use argument_factory::*;

#[tokio::main]
async fn main() {
    let _ = tokio::spawn(CurdsCronApp::new().start()).await;
}
