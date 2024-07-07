#[path = r"C:\Users\whayc\source\repos\creamery-rs\curds_core\examples\test_example.rs"]
pub mod my_test_example;
use my_test_example::main;

#[test]
fn test_example() {
    my_test_example::main();
}