use super::*;

#[cli_arguments]
#[derive(PartialEq, Debug)]
struct VecTesting {
    boolean: bool,
    option: Option<u32>,
    collection: std::vec::Vec<String>,
}

#[test]
fn parses_values_to_end_of_args() {
    let mut arguments = vec![
        "three".to_string(),
        "two".to_string(),
        "one".to_string(),
        "-collection".to_string(),
    ];
    
    assert_eq!(VecTesting { boolean: false, option: None, collection: vec!["one".to_string(), "two".to_string(), "three".to_string()] }, VecTesting::parse(&mut arguments).unwrap());
}

#[test]
fn stops_parsing_with_terminator() {
    let mut arguments = vec![
        "1234".to_string(),
        "-option".to_string(),
        "-boolean".to_string(),
        "--".to_string(),
        "three".to_string(),
        "two".to_string(),
        "one".to_string(),
        "-collection".to_string(),
    ];
    
    assert_eq!(VecTesting { boolean: true, option: Some(1234), collection: vec!["one".to_string(), "two".to_string(), "three".to_string()] }, VecTesting::parse(&mut arguments).unwrap());
}