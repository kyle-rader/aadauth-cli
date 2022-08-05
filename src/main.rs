use clap::Parser;

#[derive(Debug, Parser)]
enum Args {
    Auth,
    Clear,
}

fn main() {
    match Args::parse() {
        Args::Auth => println!("We're going to auth"),
        Args::Clear => println!("We're going to clear"),
    }
}
