use curds_cli_derive::CliArguments;
use curds_cli_core::*;

#[derive(CliArguments)]
#[name("Parsing Tests")]
#[derive(Debug, PartialEq)]
enum ParsingOperation {
    #[name("A boolean flag")]
    #[key("flag", "F", "Bool")]
    Flag,
    #[name("A string value")]
    #[key("value")]
    #[key("V", "VAL")]
    Value(String),
}

fn test_parse_flag(key: &str) {
    let mut arguments = ArgumentCollection::new(Vec::<String>::new());

    let actual = ParsingOperation::parse(String::from(key), &mut arguments).unwrap();

    assert_eq!(ParsingOperation::Flag, actual)
}

#[test]
fn parses_flags_properly() {
    test_parse_flag("Flag");
    test_parse_flag("f");
    test_parse_flag("BOOL");
}

fn test_parse_value(key: &str) {
    let mut arguments = ArgumentCollection::new(vec![String::from("TestValue")]);

    let actual = ParsingOperation::parse(String::from(key), &mut arguments).unwrap();

    assert_eq!(ParsingOperation::Value(String::from("TestValue")), actual)
}

#[test]
fn parses_value_properly() {
    test_parse_value("V");
    test_parse_value("val");
    test_parse_value("VALUE");
}