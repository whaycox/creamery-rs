use super::*;
pub use chrono::{DateTime, Local, Utc};

#[whey_mock]
pub trait Clock {
    fn current(&self) -> DateTime<Local>;
    fn current_utc(&self) -> DateTime<Utc>;
}

pub struct MachineClock;
impl Clock for MachineClock {
    fn current(&self) -> DateTime<Local> { Local::now() }
    fn current_utc(&self) -> DateTime<Utc> { Utc::now() }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn local_returns_now() {
        let clock = MachineClock;
        let before = Local::now();

        let actual = clock.current();

        let after = Local::now();
        assert!(before < actual);
        assert!(actual < after);
    }

    #[test]
    fn utc_returns_now() {
        let clock = MachineClock;
        let before = Utc::now();

        let actual = clock.current_utc();

        let after = Utc::now();
        assert!(before < actual);
        assert!(actual < after);
    }
}