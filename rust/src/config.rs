use anyhow::{anyhow, Context, Result};
use std::{env, path::PathBuf};

use crate::opts::Opts;

#[derive(Debug)]
pub struct Config {
    pub operation: Operation,
    pub pwd: PathBuf,
    pub config: PathBuf,
}

impl TryFrom<Opts> for Config {
    type Error = anyhow::Error;

    fn try_from(opts: Opts) -> std::prelude::v1::Result<Self, Self::Error> {
        let operation = opts.args.try_into()?;
        let config = get_config(opts.config)?;
        let pwd = get_pwd(opts.pwd)?;

        return Ok(Config {
            operation,
            config,
            pwd,
        });
    }
}

#[derive(Debug, PartialEq)]
pub enum Operation {
    Print(Option<String>),
    Add(String, String),
    Remove(String),
}
impl TryFrom<Vec<String>> for Operation {
    type Error = anyhow::Error;

    fn try_from(value: Vec<String>) -> Result<Self> {
        let mut value = value;

        if value.len() == 0 {
            return Ok(Operation::Print(None));
        }

        let term = value.get(0).expect("Expect to exists !");

        if term == "add" {
            if value.len() != 3 {
                return Err(anyhow!(
                    "Operation add expect 2 arguments but got {}",
                    value.len() - 1
                ));
            }

            let mut drain = value.drain(1..=2);
            return Ok(Operation::Add(
                drain.next().expect("to exist"),
                drain.next().expect("to exist"),
            ));
        }

        if term == "rm" {
            if value.len() != 2 {
                return Err(anyhow!(
                    "Operation remove expect 1 arguments but got {}",
                    value.len() - 1
                ));
            }

            let arg = value.pop().expect("to exists");
            return Ok(Operation::Remove(arg));
        }

        if value.len() > 1 {
            return Err(anyhow!(
                "Operation print expect 0 or 1 arguments but got {}",
                value.len() - 1
            ));
        }

        let arg = value.pop().expect("to exist");
        return Ok(Operation::Print(Some(arg)));
    }
}

fn get_config(config: Option<PathBuf>) -> Result<PathBuf> {
    if let Some(v) = config {
        return Ok(v);
    }

    let loc = env::var("XDG_CONFIG_HOME").context("Unable to get XDG_CONFIG_HOME")?;

    let mut loc = PathBuf::from(loc);

    loc.push("projector");
    loc.push("projector.json");

    return Ok(loc);
}

fn get_pwd(pwd: Option<PathBuf>) -> Result<PathBuf> {
    if let Some(pwd) = pwd {
        return Ok(pwd);
    }

    return Ok(std::env::current_dir().context("Errored getting current directory")?);
}

#[cfg(test)]
mod test {
    use crate::{
        config::{Config, Operation},
        opts::Opts,
    };
    use anyhow::Result;

    #[test]
    fn test_print_all() -> Result<()> {
        let opts: Config = Opts {
            args: vec![],
            pwd: None,
            config: None,
        }
        .try_into()?;
        assert_eq!(opts.operation, Operation::Print(None));
        Ok(());
    }
}
