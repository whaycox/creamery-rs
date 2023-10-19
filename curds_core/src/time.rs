use super::*;
use chrono::{DateTime, Utc};

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
    #[mock_default_return(|| UniversalTime::now())]
    fn current(&self) -> UniversalTime;
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

    #[whey(TestingContext ~ context)]
    fn returns_now() {
        let before = Utc::now();

        let actual = context
            .test_type()
            .current();

        let after = Utc::now();
        assert!(before < actual.time);
        assert!(actual.time < after);
    }
}