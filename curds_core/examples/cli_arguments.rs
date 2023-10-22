use curds_core::cli::Cli;
use curds_core_macro::cli_arguments;

#[cli_arguments]
enum TestOperations {
    Boolean,
}

fn main() {
    let operations = Cli::arguments::<TestOperations>();
    for operation in operations {
        match operation {
            TestOperations::Boolean => println!("Performing the Boolean operation"),
        }
    }
}