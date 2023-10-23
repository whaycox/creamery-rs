use super::*;
use chrono::{DateTime, Utc};

#[whey_mock]
pub trait Clock {
    #[mock_default_return(|| Utc::now())]
    fn current(&self) -> DateTime<Utc>;
}

#[injected]
struct MachineClock {}
impl Clock for MachineClock {
    fn current(&self) -> DateTime<Utc> { Utc::now() }
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
        assert!(before < actual);
        assert!(actual < after);
    }
}