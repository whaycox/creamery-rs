use super::*;

#[cli_arguments]
#[derive(PartialEq, Debug)]
enum BoolTesting {
    Test{ one: bool, two: std::primitive::bool },
}

#[test]
fn only_key_needed() {
    let mut arguments = vec![
        "-one".to_string(),
        "-two".to_string(),
        "--test".to_string(),
    ];
    
    assert_eq!(BoolTesting::Test{ one: true, two: true }, BoolTesting::parse(&mut arguments).unwrap());
}

#[test]
fn false_when_not_supplied() {
    let mut arguments = vec![
        "--test".to_string(),
    ];
    
    assert_eq!(BoolTesting::Test{ one: false, two: false }, BoolTesting::parse(&mut arguments).unwrap());
}

#[test]
fn usage_is_expected() {
    assert_eq!("[--test [-one] [-two]]", BoolTesting::usage());
}