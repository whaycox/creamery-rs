use std::collections::HashSet;

use super::*;

#[cli_arguments]
#[derive(PartialEq, Debug)]
enum TestOperations {
    Boolean,
    Unnamed(String),
    OtherUnnamed(u32),
    Named { one: u32, two: String },
}

#[test]
fn parses_boolean() {
    let mut arguments = vec![
        "--boolean".to_string(),
    ];

    assert_eq!(TestOperations::Boolean, TestOperations::parse(&mut arguments).unwrap());
}

#[test]
fn parses_unnamed() {
    let mut arguments = vec![
        "unnamed value".to_string(),
        "--unnamed".to_string(),
    ];

    assert_eq!(TestOperations::Unnamed("unnamed value".to_string()), TestOperations::parse(&mut arguments).unwrap());
}

#[test]
fn unnamed_without_value_is_error() {
    let mut arguments = vec![
        "--unnamed".to_string(),
    ];

    TestOperations::parse(&mut arguments).unwrap_err();
}

#[test]
fn unnamed_unparseable_value_is_error() {
    let mut arguments = vec![
        "non-numeric value".to_string(),
        "--other_unnamed".to_string(),
    ];

    TestOperations::parse(&mut arguments).unwrap_err();
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

    assert_eq!(TestOperations::Named{ one: 1234, two: "named value".to_string() }, TestOperations::parse(&mut arguments).unwrap());
}

#[test]
fn named_without_value_is_error() {
    let mut arguments = vec![
        "-one".to_string(),
        "named value".to_string(),
        "-two".to_string(),
        "--named".to_string(),
    ];

    TestOperations::parse(&mut arguments).unwrap_err();
}

#[test]
fn named_unparseable_value_is_error() {
    let mut arguments = vec![
        "false".to_string(),
        "-one".to_string(),
        "named value".to_string(),
        "-two".to_string(),
        "--named".to_string(),
    ];

    TestOperations::parse(&mut arguments).unwrap_err();
}

#[test]
fn named_missing_field_is_error() {
    let mut arguments = vec![
        "1234".to_string(),
        "-one".to_string(),
        "--named".to_string(),
    ];

    TestOperations::parse(&mut arguments).unwrap_err();
}

#[test]
fn usage_is_expected() {
    assert_eq!("[--boolean] [--unnamed <String>] [--other_unnamed <u32>] [--named -one <u32> -two <String>]", TestOperations::usage());
}