fn main() {
    println!("app starts");
    let operations = parse_args();
    for operation in operations {
        println!("parsed: {:?}", operation);
    }
}

#[derive(Debug)]
enum CurdsCronOperation {
    Generate(String),
    Process(String),
}

use std::env::args;

fn parse_args() -> Vec<CurdsCronOperation> {
    let mut args: Vec<String> = args()
        .skip(1)
        .collect();
    args.reverse();
    let mut operations = Vec::<CurdsCronOperation>::new();
    while args.len() > 0 {
        operations.push(parse_operation(&mut args));
    }
    operations
}

fn parse_operation(args: &mut Vec<String>) -> CurdsCronOperation {
    let operation = args.pop().unwrap();
    match operation.as_str() {
        "process" => {
            if args.len() == 0 {
                panic!("No value but process expects one");
            }
            let process_value = args.pop().unwrap();
            CurdsCronOperation::Process(process_value)
        },
        "generate" => {
            if args.len() == 0 {
                panic!("No value but generate expects one");
            }
            let generate_value = args.pop().unwrap();
            CurdsCronOperation::Generate(generate_value)            
        }
        _ => panic!("Unsupported operation: {}", operation)
    }
}