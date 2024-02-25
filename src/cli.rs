use crate::prelude::*;
use clap::Subcommand;

#[derive(Debug, Parser)]
#[command(version, about, long_about = "")]
pub(crate) struct Args {
    pub(crate) name: String,

    #[command(subcommand)]
    pub(crate) subcommand: Command,
}

#[derive(Debug, Subcommand)]
pub(crate) enum Command {
    Build(Build),
    Run(Run),
    Exec(Exec),
    Stop(Stop),
}

#[derive(Debug, Parser)]
pub(crate) struct Build {}

#[derive(Debug, Parser)]
pub(crate) struct Run {
    /// volumes to mount to the container
    #[arg(short = 'v', long = "volume")]
    pub(crate) volumes: Vec<Mount>,

    /// ports to mount to the container
    #[arg(short = 'p', long = "port")]
    pub(crate) ports: Vec<Mount>,
}

#[derive(Debug, Parser)]
pub(crate) struct Exec {
    /// use bash insertion instead of fish
    #[arg(short, long)]
    pub(crate) bash: bool,
}

#[derive(Debug, Parser)]
pub(crate) struct Stop {}

#[derive(Debug, Clone, Eq, PartialEq)]
pub(crate) struct Mount {
    pub(crate) host: String,
    pub(crate) container: String,
}

impl std::str::FromStr for Mount {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let (host, container) = s.split_once(':').ok_or_else(|| {
            error!("failed to get mounting from input `{s}`, should contian a `:`")
        })?;

        if container.len() == 0 {
            bail!("container path was empty in mounting");
        }

        if host.len() == 0 {
            bail!("container path was empty in mounting");
        }

        let mount = Mount {
            host: host.into(),
            container: container.into(),
        };

        Ok(mount)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mount_1() {
        let input = "$PWD:/path";
        let expected_out = Mount {
            host: "$PWD".into(),
            container: "/path".into(),
        };

        let output: Mount = input.parse().unwrap();

        dbg!(&output);

        assert!(output == expected_out);
    }

    #[test]
    fn mount_2() {
        let input = "$PWD:";
        assert!(input.parse::<Mount>().is_err());
    }

    #[test]
    fn mount_3() {
        let input = ":$PWD";
        assert!(input.parse::<Mount>().is_err());
    }
}
