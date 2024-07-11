use crate::time::{Clock, MachineClock};
use log::{Log, Record, Level, Metadata};

pub struct SimpleLogger<TClock : Clock> {
    clock: TClock
}

static LOGGER: SimpleLogger<MachineClock> = SimpleLogger { clock: MachineClock };
pub fn initialize() {
    log::set_logger(&LOGGER)
        .map(|()| log::set_max_level(log::LevelFilter::Info))
        .unwrap();
}

impl<TClock> Log for SimpleLogger<TClock> where
TClock : Clock + Send + Sync {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Info
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            println!("{}|{:5}|{}", self.clock.current().format("%Y-%m-%d|%H:%M:%S%.3f%:z"), record.level(), record.args());
        }
    }

    fn flush(&self) {}
}