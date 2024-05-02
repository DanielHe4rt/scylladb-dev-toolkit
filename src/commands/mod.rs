use std::fmt;

use clap::ValueEnum;

pub mod keyspace_command;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum Commands {
    Keyspace,
}

impl fmt::Display for Commands {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Commands::Keyspace => write!(f, "New Keyspace"),
        }
    }
}