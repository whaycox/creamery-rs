use super::*;

#[cli_arguments]
#[derive(PartialEq, Debug)]
enum UnnamedVecTesting {
    Leading(Vec<String>, u32),
    Trailing(bool, Vec<String>),
}

#[test]
fn parses_values_to_end_of_args() {
    let mut arguments = vec![
        "three".to_string(),
        "two".to_string(),
        "one".to_string(),
        "false".to_string(),
        "--trailing".to_string(),
    ];
    
    assert_eq!(UnnamedVecTesting::Trailing(false, vec!["one".to_string(), "two".to_string(), "three".to_string()]), UnnamedVecTesting::parse(&mut arguments).unwrap());
}

#[test]
fn stops_parsing_with_terminator() {
    let mut arguments = vec![
        "1234".to_string(),
        "--".to_string(),
        "three".to_string(),
        "two".to_string(),
        "one".to_string(),
        "--leading".to_string(),
    ];
    
    assert_eq!(UnnamedVecTesting::Leading(vec!["one".to_string(), "two".to_string(), "three".to_string()], 1234), UnnamedVecTesting::parse(&mut arguments).unwrap());
}

#[test]
fn unnamed_vec_usage_is_expected() {
    assert_eq!("[--leading <String>* -- <u32>] [--trailing <bool> <String>* --]", UnnamedVecTesting::usage());
}