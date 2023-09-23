use super::*;
use chrono::{DateTime, Utc};

#[derive(Clone)]
pub struct UniversalTime {
    pub time: DateTime<Utc>,
}
impl UniversalTime {
    pub fn now() -> Self {
        UniversalTime { 
            time: Utc::now(), 
        }
    }
}

#[whey_mock]
pub trait Clock {
    fn current(&self) -> UniversalTime;
}
impl DummyDefault for UniversalTime {
    fn dummy() -> Self { UniversalTime::now() }
}

#[injected]
struct MachineClock {}
impl Clock for MachineClock {
    fn current(&self) -> UniversalTime { UniversalTime::now() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[whey_context(MachineClock)]
    struct TestingContext {}

    #[whey]
    fn returns_now(context: TestingContext) {
        let clock: MachineClock = context.generate();
        let before = Utc::now();

        let actual = clock.current();

        let after = Utc::now();
        assert!(before < actual.time);
        assert!(actual.time < after);
    }
}