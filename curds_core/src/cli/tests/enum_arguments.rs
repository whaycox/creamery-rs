use super::*;

#[cli_arguments]
#[derive(PartialEq, Debug)]
enum TestOperations {
    Boolean,
    Unnamed(String),
    Named { one: u32, two: String },
}

#[test]
fn parses_boolean() {
    let mut arguments = vec![
        "--boolean".to_string(),
    ];

    assert_eq!(TestOperations::Boolean, TestOperations::parse(&mut arguments));
}

#[test]
fn parses_unnamed() {
    let mut arguments = vec![
        "unnamed value".to_string(),
        "--unnamed".to_string(),
    ];

    assert_eq!(TestOperations::Unnamed("unnamed value".to_string()), TestOperations::parse(&mut arguments));
}

#[test]
fn parses_named() {
    let mut arguments = vec![
        "1234".to_string(),
        "-one".to_string(),
        "named value".to_string(),
        "-two".to_string(),
        "--named".to_string(),
    ];

    assert_eq!(TestOperations::Named{ one: 1234, two: "named value".to_string() }, TestOperations::parse(&mut arguments));
}