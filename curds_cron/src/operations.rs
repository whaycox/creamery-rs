use super::*;

#[cli_arguments]
pub enum CronOperation {
    Test(Vec<CronExpression>),
    Generate { path: Option<String> },
    Start { path: Option<String> }
}