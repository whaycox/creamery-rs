use curds_core::cron::CurdsCronFieldParser;

use super::*;
use std::marker::PhantomData;

pub struct CurdsCronApp<TFactory: ArgumentFactory, TParser: CronFieldParser> {
    arguments: TFactory,
    parser_phantom: PhantomData<TParser>,
}

impl CurdsCronApp<CronArgumentFactory, CurdsCronFieldParser> {
    pub fn new() -> Self {
        Self {
            arguments: CronArgumentFactory,
            parser_phantom: PhantomData,
        }
    }
}

impl<TFactory, TParser> CurdsCronApp<TFactory, TParser>
where TFactory : ArgumentFactory,
TParser : CronFieldParser {
    pub async fn start(self) {
        println!("starting");
        for arg in self.arguments.args() {
            let expression: CronExpression = CronExpression::parse::<TParser>(&arg).unwrap();
            println!("{:?}", expression);
        }
        println!("finished");
    }
}