use curds_core::cli::{Cli, cli_arguments};

#[cli_arguments]
#[description("An example of building out CLI arguments with just an attribute.")]
enum TestOperations {
    #[description("A boolean operation, it contains no other data.")]
    Boolean,
    #[description("An operation with unnamed parameters, they must be provided in order.")]
    Unnamed(String, bool, u32),
    #[description("An operation with named parameters, they can be provided in any order.")]
    Named { optional: Option<u32>, bit: bool, collection: Vec<String> }
}

fn main() {
    curds_core::logger::initialize();
    for operation in Cli::arguments::<TestOperations>() {
        match operation {
            TestOperations::Boolean => println!("Performing the Boolean operation"),
            TestOperations::Unnamed(str, bit, int) => println!("Performing the Unnamed operation \"{}\", {}, {}", str, bit, int),
            TestOperations::Named { optional, bit, collection } => println!("Performing the Named operation {:?}, {}, {:?}", optional, bit, collection),
        }
    }
}