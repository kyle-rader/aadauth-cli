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
    match args {
        Args::Auth(Target {
            scopes,
            client,
            tenant,
        }) => vec![
            String::from("--client"),
            client,
            String::from("--resource"),
            String::from(""),
            String::from("--scopes"),
            scopes[0].clone(),
            String::from("--tenant"),
            tenant,
        ],
        Args::Clear(Target {
            scopes,
            client,
            tenant,
        }) => vec![],
    }
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
    use super::{translate, Args, Target};

    #[test]
    fn auth_command() {
        // let args = Args::parse_from(&[
        //     "azureauth",
        //     "auth",
        //     "--client",
        //     "foo",
        //     "--scopes",
        //     "scope1",
        //     "--tenant",
        //     "contoso",
        // ]);

        let args = Args::Auth(Target {
            scopes: vec![String::from("scope1")],
            client: String::from("foo"),
            tenant: String::from("contoso"),
        });

        let expected = vec![
            "--client",
            "foo",
            "--resource",
            "",
            "--scopes",
            "scope1",
            "--tenant",
            "contoso",
        ];

        let subject = translate(args);
        assert_eq!(subject, expected);
    }
}
