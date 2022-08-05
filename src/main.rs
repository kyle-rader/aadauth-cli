use std::process::Command;

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

fn translate(args: Args) -> Vec<String> {
    todo!()
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

#[cfg(test)]
mod tests {
    use std::process::Command;

    use clap::Parser;

    use super::{translate, Args};

    #[test]
    fn auth_command() {
        let args = Args::parse_from(&[
            "azureauth",
            "auth",
            "--client",
            "foo",
            "--scopes",
            "scope1",
            "--tenant",
            "contoso",
        ]);

        let expected = vec![
            String::from("--client"),
            String::from("foo"),
            String::from("--resource"),
            String::from(""),
            String::from("--scopes"),
            String::from("scope1"),
            String::from("--tenant"),
            String::from("contoso"),
        ];

        let subject = translate(args);
        assert_eq!(subject, expected);
    }
}
