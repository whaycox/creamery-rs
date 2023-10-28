use super::*;

#[cli_arguments]
#[derive(PartialEq, Debug)]
struct OptionVecTesting {
    boolean: bool,
    optional_collection: std::option::Option<std::vec::Vec<String>>,
}

#[test]
fn parses_values_to_end_of_args() {
    let mut arguments = vec![
        "three".to_string(),
        "two".to_string(),
        "one".to_string(),
        "-optional_collection".to_string(),
    ];
    
    assert_eq!(OptionVecTesting { boolean: false, optional_collection: Some(vec!["one".to_string(), "two".to_string(), "three".to_string()]) }, OptionVecTesting::parse(&mut arguments).unwrap());
}

#[test]
fn stops_parsing_with_terminator() {
    let mut arguments = vec![
        "-boolean".to_string(),
        "--".to_string(),
        "three".to_string(),
        "two".to_string(),
        "one".to_string(),
        "-optional_collection".to_string(),
    ];
    
    assert_eq!(OptionVecTesting { boolean: true, optional_collection: Some(vec!["one".to_string(), "two".to_string(), "three".to_string()]) }, OptionVecTesting::parse(&mut arguments).unwrap());
}

#[test]
fn parses_none_when_not_supplied() {
    let mut arguments: Vec<String> = Vec::new();
    
    assert_eq!(OptionVecTesting { boolean: false, optional_collection: None }, OptionVecTesting::parse(&mut arguments).unwrap());
}

#[test]
fn optional_vec_usage_is_expected() {
    assert_eq!("[-boolean] [-optional_collection <String>* --]", OptionVecTesting::usage());
}