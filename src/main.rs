use std::env;

fn main() {
    let mut name = "India";
    let args: Vec<String> = env::args().collect();

    if args.len() == 2 {
        name =  &args[1];
    }

    println!("Hello, {name}");
}
