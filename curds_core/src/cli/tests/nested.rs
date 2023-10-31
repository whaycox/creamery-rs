use super::*;

#[cli_arguments]
#[derive(PartialEq, Debug)]
enum NestedOperations {
    Outer(NestedStruct),
}

#[cli_arguments]
#[derive(PartialEq, Debug)]
struct NestedStruct {
    bit: bool,
    collection: Option<Vec<u32>>,
    option: Option<String>,
}

#[test]
fn parses_nested_enum() {
    let mut arguments = vec![
        "-bit".to_string(),
        "--".to_string(),
        "3".to_string(),
        "2".to_string(),
        "1".to_string(),
        "-collection".to_string(),
        "a test message".to_string(),
        "-option".to_string(),
        "--outer".to_string(),
    ];
    
    assert_eq!(NestedOperations::Outer(NestedStruct { bit: true, collection: Some(vec![1, 2, 3]), option: Some("a test message".to_string()) }), NestedOperations::parse(&mut arguments).unwrap());
}


#[test]
fn nested_usage_is_expected() {    
    assert_eq!("[--outer [-bit] [-collection <u32>* --] [-option <String>]]", NestedOperations::usage());
}
