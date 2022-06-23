use super::*;
use chrono::{DateTime, Utc};

pub trait Clock {
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

    #[whey]
    fn returns_now(context: TestingContext) {
        let before = Utc::now();

        let actual = context
            .generate()
            .current();

        let after = Utc::now();
        assert!(before < actual);
        assert!(actual < after);
    }
}