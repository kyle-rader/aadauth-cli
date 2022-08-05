use clap::Parser;

#[derive(Debug, Parser)]
struct Target {
    /// Scopes to request
    #[clap(long, required = true)]
    scopes: Vec<String>,
    /// Client ID
    #[clap(long)]
    client: String,
    /// Tenant ID
    #[clap(long)]
    tenant: String,
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
        Args::Auth(Target {
            scopes,
            client,
            tenant,
        }) => println!("We're going to auth with c:{client}, s:{scopes:?}, in t:{tenant}"),
        Args::Clear(Target {
            scopes,
            client,
            tenant,
        }) => println!("We're going to clear tokens for c:{client}, s:{scopes:?}, in t:{tenant}"),
    }
}
