use std::any::TypeId;
use std::default::Default;
use super::dependency_injection::Injected;

#[derive(Default)]
pub struct WheySynchronizer {
    sequence: Vec<(TypeId, String)>,
}

impl<TProvider> Injected<TProvider> for WheySynchronizer {
    fn inject(_: &TProvider) -> Self { Default::default() }
}

impl WheySynchronizer {
    pub fn load(&mut self, type_id: TypeId, method: String) {
        self.sequence.push((type_id, method));
    }

    pub fn consume(&mut self, type_id: TypeId, method: &str) {
        if self.sequence.len() > 0 {
            let expected = &self.sequence[0];
            if expected.0 != type_id {
                panic!("sequence expected a TypeId of {:?} but was provided {:?}", expected.0, type_id);
            }
            if expected.1 != method {
                panic!("sequence expected a method of {} but was provided {}", expected.1, method);
            }
            self.sequence.remove(0);
        }
    }

    pub fn assert(&mut self) {
        if self.sequence.len() > 0 {
            panic!("not all sequence elements have been consumed");
        }
        self.reset();
    }

    pub fn reset(&mut self) {
        self.sequence.clear();
    }
}

impl Drop for WheySynchronizer {
    fn drop(&mut self) {
        if !std::thread::panicking() {
            self.assert();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_STRING: &str = "test_str";

    struct TestStruct {}

    #[test]
    fn load_adds_element() {
        let mut synchronizer: WheySynchronizer = Default::default();

        synchronizer.load(TypeId::of::<TestStruct>(), String::from(TEST_STRING));

        assert_eq!(1, synchronizer.sequence.len());
        assert_eq!(TypeId::of::<TestStruct>(), synchronizer.sequence[0].0);
        assert_eq!(String::from(TEST_STRING), synchronizer.sequence[0].1);
        synchronizer.reset();
    }

    #[test]
    fn consuming_without_loading_is_ok() {
        let mut synchronizer: WheySynchronizer = Default::default();

        synchronizer.consume(TypeId::of::<TestStruct>(), TEST_STRING);
    }

    #[test]
    fn consume_removes_element() {
        let mut synchronizer: WheySynchronizer = Default::default();
        synchronizer.load(TypeId::of::<TestStruct>(), String::from(TEST_STRING));

        synchronizer.consume(TypeId::of::<TestStruct>(), TEST_STRING);

        assert_eq!(0, synchronizer.sequence.len());
    }

    #[test]
    #[should_panic(expected = "sequence expected a TypeId of")]
    fn consume_panics_if_wrong_type_is_supplied() {
        let mut synchronizer: WheySynchronizer = Default::default();
        synchronizer.load(TypeId::of::<TestStruct>(), String::from(TEST_STRING));

        synchronizer.consume(TypeId::of::<WheySynchronizer>(), TEST_STRING);
    }

    #[test]
    #[should_panic(expected = "sequence expected a method of test_str but was provided TEST_STRING")]
    fn consume_panics_if_wrong_method_is_supplied() {
        let mut synchronizer: WheySynchronizer = Default::default();
        synchronizer.load(TypeId::of::<TestStruct>(), String::from(TEST_STRING));

        synchronizer.consume(TypeId::of::<TestStruct>(), "TEST_STRING");
    }

    #[test]
    #[should_panic(expected = "not all sequence elements have been consumed")]
    fn assert_panics_if_expectations_are_left() {
        let mut synchronizer: WheySynchronizer = Default::default();
        synchronizer.load(TypeId::of::<TestStruct>(), String::from(TEST_STRING));

        synchronizer.assert();
    }

    #[test]
    fn reset_clears_expectations() {
        let mut synchronizer: WheySynchronizer = Default::default();
        synchronizer.load(TypeId::of::<TestStruct>(), String::from(TEST_STRING));

        synchronizer.reset();

        assert_eq!(0, synchronizer.sequence.len());
    }

    #[test]
    #[should_panic(expected = "not all sequence elements have been consumed")]
    fn drop_asserts() {
        let mut synchronizer: WheySynchronizer = Default::default();
        synchronizer.load(TypeId::of::<TestStruct>(), String::from(TEST_STRING));
    }
}