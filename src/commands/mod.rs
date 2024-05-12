use std::fmt;

use clap::Subcommand;

pub mod keyspace_command;
pub mod setup_multi_dc_command;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Subcommand)]
pub enum Command {
    /// Create a new Keyspace inside a ScyllaDB Cluster
    Keyspace {
        /// Keyspace name
        #[arg(short, long, default_value = "mykeyspace")]
        keyspace: String,

        /// Replication factor
        #[arg(short, long, default_value_t = 3)]
        replication_factor: u8,

        /// Drop the keyspace if it already exists
        #[arg(short, long)]
        drop: bool,
    },
    MultiDC {
        /// Replication factor
        #[arg(short, long, default_value_t = 3)]
        replication_factor: u8,

        #[clap(short, long, value_parser, num_args = 1.., value_delimiter = ',', default_value = "DC1,DC2")]
        dcs: Vec<String>,

        #[arg(short, long, default_value = "mykeyspace")]
        keyspace: String,
    },

}

impl fmt::Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Command::Keyspace { .. } => write!(f, "New Keyspace"),
            Command::MultiDC { .. } => { write!(f, "Multi DC Setup") }
        }
    }
}