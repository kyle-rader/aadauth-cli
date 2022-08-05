use clap::Parser;

#[derive(Debug, Parser)]
struct Target {
    /// Scopes to request
    #[clap(long)]
    scopes: Option<Vec<String>>,
    /// Client ID
    #[clap(long)]
    client: Option<String>,
    /// Tenant ID
    #[clap(long)]
    tenant: Option<String>,
}

/// Do Auth Well
#[derive(Debug, Parser)]
#[clap(version)]
enum Args {
    /// Do the auth well
    Auth(Target),
    /// No more token
    Clear(Target),
}

fn main() {
    match Args::parse() {
        Args::Auth(_) => println!("We're going to auth"),
        Args::Clear(_) => println!("We're going to clear"),
    }
}
