use super::*;

pub struct ArgumentCollection {
    arguments: Vec<String>,
}
impl ArgumentCollection {
    pub fn new(mut arguments: Vec<String>) -> Self {
        arguments.reverse();
        ArgumentCollection {
            arguments: arguments,
        }
    }

    pub fn has_values(&self) -> bool {
        self.arguments.len() > 0
    }

    pub fn pop(&mut self) -> CliParseResult<String> {
        if let Some(argument) = self.arguments.pop() {
            Ok(argument)
        }
        else {
            Err(CliParseError::MissingValue)
        }
    }
}

mod tests {
    use super::*;

    fn setup_collection(is_empty: bool) -> ArgumentCollection {
        if is_empty {
            ArgumentCollection::new(Vec::<String>::new())
        }
        else {
            let test_strings = vec![String::from("one"), String::from("two"), String::from("three")];
            ArgumentCollection::new(test_strings)
        }
    }

    #[test]
    fn new_reverses_input() {
        let test_object = setup_collection(false);

        assert_eq!("three", test_object.arguments[0]);
        assert_eq!("two", test_object.arguments[1]);
        assert_eq!("one", test_object.arguments[2]);
    }

    #[test]
    fn empty_collection_has_no_values() {
        let test_object = setup_collection(true);

        assert_eq!(false, test_object.has_values());
    }

    #[test]
    fn populated_collection_has_values() {
        let test_object = setup_collection(false);

        assert_eq!(true, test_object.has_values());
    }

    #[test]
    fn pop_without_values_returns_error() {
        let mut test_object = setup_collection(true);

        let actual = test_object.pop().expect_err("Expected a missing value error");

        assert_eq!(CliParseError::MissingValue, actual);
    }

    #[test]
    fn pop_with_values_returns_in_input_order() {
        let mut test_object = setup_collection(false);

        assert_eq!("one", test_object.pop().unwrap());
        assert_eq!("two", test_object.pop().unwrap());
        assert_eq!("three", test_object.pop().unwrap());
    }
}