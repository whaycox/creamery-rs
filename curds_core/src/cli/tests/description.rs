use super::*;

#[cli_arguments]
#[derive(PartialEq, Debug)]
#[description("A test of the descriptions attribute.")]
enum DescriptionTest {
    #[description("An operation that either is or is not provided.")]
    BooleanOperation,
}

#[test]
fn description_test() {
    let actual = DescriptionTest::description().unwrap();

    assert_eq!(4, actual.len());
    assert_eq!("A test of the descriptions attribute.", actual[0]);
    assert_eq!("Operations:", actual[1]);
    assert_eq!("--boolean_operation", actual[2]);
    assert_eq!("An operation that either is or is not provided.", actual[3]);
}