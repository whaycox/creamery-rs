use super::*;

#[cli_arguments]
#[derive(PartialEq, Debug)]
enum TestOperations {
    FirstBoolean,
    SecondBool,
    Message(String, u32),
    Point { x: u32, y: u32 },
}

#[whey_context(CliArgumentParser)]
#[mocks(dyn ArgumentFactory)]
#[mocks(dyn Terminal)]
struct CliArgumentParserContext {}

#[whey(CliArgumentParserContext ~ context)]
fn parses_boolean_operations() {
    mock_return!(context ~ ArgumentFactory ~ create, || vec![
        "--first_boolean".to_string(),
        "--second_bool".to_string(),
    ], 1);

    let actual = context
        .test_type()
        .parse()
        .unwrap();

    assert_eq!(2, actual.len());
    assert_eq!(TestOperations::FirstBoolean, actual[0]);
    assert_eq!(TestOperations::SecondBool, actual[1]);
}

#[whey(CliArgumentParserContext ~ context)]
fn parses_operations_with_anonymous_values() {
    mock_return!(context ~ ArgumentFactory ~ create, || vec![
        "--message".to_string(),
        "This is a test message".to_string(),
        "123".to_string(),
    ], 1);

    let actual = context
        .test_type()
        .parse()
        .unwrap();

    assert_eq!(1, actual.len());
    assert_eq!(TestOperations::Message(String::from("This is a test message"), 123), actual[0]);
}

#[whey(CliArgumentParserContext ~ context)]
fn parses_operations_with_named_values() {
    mock_return!(context ~ ArgumentFactory ~ create, || vec![
        "--point".to_string(),
        "-y".to_string(),
        "123".to_string(),
        "-x".to_string(),
        "234".to_string(),
    ], 1);

    let actual = context
        .test_type()
        .parse()
        .unwrap();

    assert_eq!(1, actual.len());
    assert_eq!(TestOperations::Point{ x: 234, y: 123 }, actual[0]);
}

const TEST_APPLICATION_NAME: &str = "TestApplication";
#[whey(CliArgumentParserContext ~ context)]
fn parse_error_writes_usage() {
    mock_sequence!(context ~ [
        ArgumentFactory ~ create() -> || vec![
            "NOT_PARSEABLE".to_string(),
        ],
        ArgumentFactory ~ application_name() -> || TEST_APPLICATION_NAME.to_string(),
        Terminal ~ write(|input| input == "TestApplication [--first_boolean] [--second_bool] [--message <String> <u32>] [--point -x <u32> -y <u32>]"),
    ]);

    context
        .test_type()
        .parse::<TestOperations>()
        .unwrap_err();
}