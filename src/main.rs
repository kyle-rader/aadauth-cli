use std::process::Command;

use clap::Parser;

#[derive(Debug, Parser)]
struct Target {
    /// Client ID
    #[clap(long)]
    client: String,
    /// Tenant ID
    #[clap(long)]
    tenant: String,
    /// Scopes to request
    #[clap(long, required = true)]
    scopes: Vec<String>,
}

impl From<Target> for Vec<String> {
    fn from(target: Target) -> Self {
        let mut args = vec![
            String::from("--client"),
            target.client,
            String::from("--tenant"),
            target.tenant,
            String::from("--resource"),
            String::from(""),
        ];

        for scope in target.scopes {
            args.push(String::from("--scopes"));
            args.push(scope);
        }

        args
    }
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
        Args::Auth(target) => target.into(),
        Args::Clear(target) => {
            let mut args: Vec<String> = target.into();
            args.push(String::from("--clear"));
            args
        }
    }
}

fn main() {
    let args = Args::parse();
    let args = translate(args);
    let result = Command::new("azureauth").args(args).spawn();

    match result {
        Ok(_) => {}
        Err(err) => {
            eprintln!("{err}");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{translate, Args, Target};
    use pretty_assertions::assert_eq;

    const EXPECTED_BASE: [&str; 8] = [
        "--client",
        "foo",
        "--tenant",
        "contoso",
        "--resource",
        "",
        "--scopes",
        "s1",
    ];

    #[test]
    fn auth_command() {
        let args = Args::Auth(Target {
            scopes: vec![String::from("s1")],
            client: String::from("foo"),
            tenant: String::from("contoso"),
        });

        let expected = EXPECTED_BASE;

        let subject = translate(args);
        assert_eq!(subject, expected);
    }

    #[test]
    fn auth_command_many_scopes() {
        let args = Args::Auth(Target {
            scopes: vec![String::from("s1"), String::from("scope2")],
            client: String::from("foo"),
            tenant: String::from("contoso"),
        });

        let mut expected = EXPECTED_BASE.to_vec();
        expected.extend_from_slice(&["--scopes", "scope2"]);

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

        let mut expected = EXPECTED_BASE.to_vec();
        expected.push("--clear");

        let subject = translate(args);
        assert_eq!(subject, expected);
    }

    #[test]
    fn clear_command_many_scopes() {
        let args = Args::Clear(Target {
            scopes: vec![String::from("s1"), String::from("s2")],
            client: String::from("foo"),
            tenant: String::from("contoso"),
        });

        let mut expected = EXPECTED_BASE.to_vec();
        expected.extend_from_slice(&["--scopes", "s2", "--clear"]);

        let subject = translate(args);
        assert_eq!(subject, expected);
    }
}
