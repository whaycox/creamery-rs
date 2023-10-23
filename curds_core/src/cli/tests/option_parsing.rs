use super::*;

#[cli_arguments]
#[derive(PartialEq, Debug)]
enum OptionTesting {
    Test{ one: Option<u32>, two: std::option::Option<bool> },
}

#[test]
fn parses_optional_type() {
    let mut arguments = vec![
        "9876".to_string(),
        "-one".to_string(),
        "false".to_string(),
        "-two".to_string(),
        "--test".to_string(),
    ];
    
    assert_eq!(OptionTesting::Test{ one: Some(9876), two: Some(false) }, OptionTesting::parse(&mut arguments).unwrap());
}

#[test]
fn none_when_not_supplied() {
    let mut arguments = vec![
        "--test".to_string(),
    ];
    
    assert_eq!(OptionTesting::Test{ one: None, two: None }, OptionTesting::parse(&mut arguments).unwrap());
}