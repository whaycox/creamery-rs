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

#[cli_arguments]
#[derive(PartialEq, Debug)]
struct UnitStruct;

#[test]
fn parses_unit_struct() {
    assert_eq!(UnitStruct, UnitStruct::parse(&mut Vec::new()).unwrap());
}
