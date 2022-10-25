use std::env;
use tokio;
use randog;

#[tokio::main]
async fn main()-> Result<(), Box<dyn std::error::Error>> {
    println!("Asyncing....");
    let fact = randog::get_fact().await?;
    println!("{}",fact.facts[0]) ;
    let mut name = "India";
    let args: Vec<String> = env::args().collect();

    if args.len() == 2 {
        name =  &args[1];
    }

    println!("Hello, {name}");
    Ok(())
}
