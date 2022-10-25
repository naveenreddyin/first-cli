use std::env;
use tokio;

#[tokio::main]
async fn main() {
    println!("Asyncing....");
    let mut name = "India";
    let args: Vec<String> = env::args().collect();

    if args.len() == 2 {
        name =  &args[1];
    }

    println!("Hello, {name}");

}
