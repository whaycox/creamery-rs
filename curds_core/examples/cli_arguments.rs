use curds_core::cli::{Cli, cli_arguments};

#[cli_arguments]
enum TestOperations {
    Boolean,
    Unnamed(String, bool, u32),
    Named { optional: Option<u32>, bit: bool, collection: Vec<String> }
}

fn main() {
    for operation in Cli::arguments::<TestOperations>() {
        match operation {
            TestOperations::Boolean => println!("Performing the Boolean operation"),
            TestOperations::Unnamed(str, bit, int) => println!("Performing the Unnamed operation \"{}\", {}, {}", str, bit, int),
            TestOperations::Named { optional, bit, collection } => println!("Performing the Named operation {:?}, {}, {:?}", optional, bit, collection),
        }
    }
}