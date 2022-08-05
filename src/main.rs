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
        }) => {
            let mut result = vec![
                String::from("--client"),
                client,
                String::from("--tenant"),
                tenant,
                String::from("--resource"),
                String::from(""),
            ];

            for scope in scopes {
                result.push(String::from("--scopes"));
                result.push(scope);
            }

            result
        }
        Args::Clear(Target {
            scopes,
            client,
            tenant,
        }) => {
            let mut result = vec![
                String::from("--client"),
                client,
                String::from("--tenant"),
                tenant,
                String::from("--resource"),
                String::from(""),
            ];

            for scope in scopes {
                result.push(String::from("--scopes"));
                result.push(scope);
            }

            result.push(String::from("--clear"));

            result
        }
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
    use pretty_assertions::assert_eq;

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
            "--tenant",
            "contoso",
            "--resource",
            "",
            "--scopes",
            "scope1",
        ];

        let subject = translate(args);
        assert_eq!(subject, expected);
    }

    #[test]
    fn auth_command_many_scopes() {
        let args = Args::Auth(Target {
            scopes: vec![String::from("scope1"), String::from("scope2")],
            client: String::from("foo"),
            tenant: String::from("contoso"),
        });

        let expected = &[
            "--client",
            "foo",
            "--tenant",
            "contoso",
            "--resource",
            "",
            "--scopes",
            "scope1",
            "--scopes",
            "scope2",
        ];

        let subject = translate(args);
        assert_eq!(subject, expected);
    }

    #[test]
    fn clear_command() {
        let args = Args::Clear(Target {
            scopes: vec![String::from("s1")],
            client: String::from("foo"),
            tenant: String::from("contoso"),
        });

        let expected = &[
            "--client",
            "foo",
            "--tenant",
            "contoso",
            "--resource",
            "",
            "--scopes",
            "s1",
            "--clear",
        ];

        let subject = translate(args);
        assert_eq!(subject, expected);
    }
}
