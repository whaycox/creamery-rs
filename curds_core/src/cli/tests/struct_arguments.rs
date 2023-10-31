use super::*;

#[cli_arguments]
#[derive(PartialEq, Debug)]
struct TestStruct {
    one: u32,
    two: String,
}

#[test]
fn parses_struct() {
    let mut arguments = vec![
        "9876".to_string(),
        "-one".to_string(),
        "test struct value".to_string(),
        "-two".to_string(),
    ];

    assert_eq!(TestStruct { one: 9876, two: "test struct value".to_string()}, TestStruct::parse(&mut arguments).unwrap());
}

#[test]
fn struct_missing_value_is_error() {
    let mut arguments = vec![
        "-one".to_string(),
        "test struct value".to_string(),
        "-two".to_string(),
    ];

    TestStruct::parse(&mut arguments).unwrap_err();
}

#[test]
fn struct_unparseable_value_is_error() {
    let mut arguments = vec![
        "false".to_string(),
        "-one".to_string(),
        "test struct value".to_string(),
        "-two".to_string(),
    ];

    TestStruct::parse(&mut arguments).unwrap_err();
}

#[test]
fn struct_missing_field_is_error() {
    let mut arguments = vec![
        "9876".to_string(),
        "-one".to_string(),
    ];

    TestStruct::parse(&mut arguments).unwrap_err();
}

#[test]
fn struct_usage_is_expected() {
    assert_eq!("-one <u32> -two <String>", TestStruct::usage());
}

#[cli_arguments]
#[derive(PartialEq, Debug)]
struct TupleStruct (u32, String);

#[test]
fn parses_tuple_struct() {
    let mut arguments = vec![
        "test tuple struct value".to_string(),
        "1234".to_string(),
    ];

    assert_eq!(TupleStruct(1234, "test tuple struct value".to_string()), TupleStruct::parse(&mut arguments).unwrap());
}

#[test]
fn tuple_struct_missing_value_is_error() {
    let mut arguments = vec![
        "1234".to_string(),
    ];

    TestStruct::parse(&mut arguments).unwrap_err();
}

#[test]
fn tuple_struct_unparseable_value_is_error() {
    let mut arguments = vec![
        "test tuple struct value".to_string(),
        "false".to_string(),
    ];

    TestStruct::parse(&mut arguments).unwrap_err();
}

#[test]
fn tuple_struct_usage_is_expected() {
    assert_eq!("<u32> <String>", TupleStruct::usage());
}

#[cli_arguments]
#[derive(PartialEq, Debug)]
struct UnitStruct;

#[test]
fn parses_unit_struct() {
    assert_eq!(UnitStruct, UnitStruct::parse(&mut Vec::new()).unwrap());
}

#[test]
fn unit_struct_usage_is_expected() {
    assert_eq!("", UnitStruct::usage());
}