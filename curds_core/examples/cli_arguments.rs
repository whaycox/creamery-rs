fn main() {
    let args: Vec<_> = std::env::args().collect();
    for i in 0..args.len() {
        println!("arg{}: {}", i, args[i]);
    }    
    let other_args: Vec<_> = std::env::args().collect();
    for i in 0..other_args.len() {
        println!("arg{}: {}", i, other_args[i]);
    }
    println!("Hi there");
}