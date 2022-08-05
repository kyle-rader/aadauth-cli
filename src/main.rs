use clap::Parser;

#[derive(Debug, Parser)]
struct Target {
    #[clap(long)]
    scopes: Option<Vec<String>>,
    #[clap(long)]
    client: Option<String>,
    #[clap(long)]
    tenant: Option<String>,
}

#[derive(Debug, Parser)]
enum Args {
    Auth(Target),
    Clear(Target),
}

fn main() {
    match Args::parse() {
        Args::Auth(_) => println!("We're going to auth"),
        Args::Clear(_) => println!("We're going to clear"),
    }
}
