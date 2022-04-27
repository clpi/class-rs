use std::{ 
    env::{self, args}, 
    process::{id, self}, 
    path::{self, PathBuf}, 
    fs, io, 
    str::FromStr, convert::Infallible 
};
use serde::{Serialize, Deserialize};

#[derive(Debug, Default)]
pub struct Args {
    cmd: Cmd,
    debug: bool,
    dir: Option<PathBuf>,
    workspace: Option<String>,
    profile: Option<String>,
}
impl Args {
    pub fn parse() -> Self {
        let mut rcmd = Self::default();
        let mut ags = env::args();
        let _bin = ags.next().unwrap_or_default();
        rcmd.cmd = match Cmd::parse(ags.next()) {
            None => Cmd::Help(None),
            Some(Cmd::Help(_)) => Cmd::parse_help_sub(ags.next().clone()),
            Some(c) => c,
        };
        return rcmd;
    }
}

pub trait Argument: std::fmt::Debug + Default + FromStr {
    fn position() -> Option<usize> { None }

    fn required() -> bool { false }

    fn parse(a: Option<String>) -> Option<Self> {
        match a {
            Some(ref s) => return Self::from_str(s).ok(),
            None => {
                if Self::required() { process::exit(1) }
                else { return None };
            }
        }
    }
}

pub trait Flag: std::fmt::Debug + Default + FromStr {
    fn parse(arg: Option<String>) -> Option<Self> {
        match arg {
            Some(ref s) => return Self::from_str(s).ok(),
            None => return None,
        }
    }
}

pub trait Opt: std::fmt::Debug + Default + FromStr {

    fn value_name() -> String;

    fn value_type() -> String;

    fn parse(arg: Option<String>) -> Option<Self> {
        return Self::from_str(&arg.unwrap_or_default()).ok();

    }
}
pub trait Subcommand: std::fmt::Debug + std::str::FromStr + Default {
    fn parse(arg: Option<String>) -> Option<Self> {
        return arg.map(|s| Self::from_str(&s).unwrap_or_default());
    }
}


#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Cmd {
    Help(Option<Box<Self>>),
    Init,
    Shell,
    List,
    Sync,
    Find,
    Log,
    Add,
    Remove,
    Run,
    Profile,
}
impl Default for Cmd {
    fn default() -> Self {
        Cmd::Help(None)
    }
}
impl FromStr for Cmd {
    type Err = Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        return match s {
            "init"    | "i"      => Ok(Cmd::Init),
            "log"     | "l"      => Ok(Cmd::Log),
            "sync"    | "s"      => Ok(Cmd::Sync),
            "shell"   | "sh"     => Ok(Cmd::Shell),
            "find"    | "f"      => Ok(Cmd::Find),
            "list"    | "ls"     => Ok(Cmd::List),
            "add"     | "a"      => Ok(Cmd::Add),
            "run"     | "r"      => Ok(Cmd::Run),
            "remove"  | "rm"     => Ok(Cmd::Remove),
            "profile" | "p"      => Ok(Cmd::Profile),
            "help"    | "h" | _  => Ok(Cmd::Help(None)),
        }
    }
}
impl Subcommand for Cmd {
    fn parse(arg: Option<String>) -> Option<Self> {
        match arg {
            Some(s) => return Self::from_str(&s).ok(),
            None => return None,
        }
    }
}
impl Cmd {
    pub fn parse_help_sub(s: Option<String>) -> Self {
        return match s {
            None => Cmd::Help(None),
            Some(c) => match Cmd::parse(Some(c.to_string())) {
                None => Cmd::Help(None),
                Some(cmd) => Cmd::Help(Some(Box::new(cmd))),
            }
        }
    }
}

