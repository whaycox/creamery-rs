use super::*;

impl CliArgumentParser<TestingArgumentFactory> {
    pub fn test_object() -> Self {
        Self {
            factory: TestingArgumentFactory::new()
        }
    }
}

#[cli_arguments]
#[derive(PartialEq, Debug)]
enum TestOperations {
    FirstBoolean,
    SecondBool,
    Message(String, u32),
    Point { x: u32, y: u32 },
}

#[test]
fn parses_boolean_operations() {
    let test_object = CliArgumentParser::test_object();
    test_object.factory.store_return_create(|| vec![
        "--first_boolean".to_string(),
        "--second_bool".to_string(),
    ], 1);

    let actual = test_object
        .parse()
        .unwrap();

    assert_eq!(2, actual.len());
    assert_eq!(TestOperations::FirstBoolean, actual[0]);
    assert_eq!(TestOperations::SecondBool, actual[1]);
}

#[test]
fn parses_operations_with_anonymous_values() {
    let test_object = CliArgumentParser::test_object();
    test_object.factory.store_return_create(|| vec![
        "--message".to_string(),
        "This is a test message".to_string(),
        "123".to_string(),
    ], 1);

    let actual = test_object
        .parse()
        .unwrap();

    assert_eq!(1, actual.len());
    assert_eq!(TestOperations::Message(String::from("This is a test message"), 123), actual[0]);
}

#[test]
fn parses_operations_with_named_values() {
    let test_object = CliArgumentParser::test_object();
    test_object.factory.store_return_create(|| vec![
        "--point".to_string(),
        "-y".to_string(),
        "123".to_string(),
        "-x".to_string(),
        "234".to_string(),
    ], 1);

    let actual = test_object
        .parse()
        .unwrap();

    assert_eq!(1, actual.len());
    assert_eq!(TestOperations::Point{ x: 234, y: 123 }, actual[0]);
}

#[test]
fn usage_is_expected() {
    assert_eq!("[--first_boolean] [--second_bool] [--message <String> <u32>] [--point -x <u32> -y <u32>]", TestOperations::usage());
}