use std::fmt;

use clap::{Args, Subcommand};

pub mod keyspace_command;
pub mod setup_multi_dc_command;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Args)]
pub struct ConnectionConfig {
    /// Scylla Host
    #[arg(long, default_value = "localhost:9042")]
    pub host: String,

    #[arg(short, long, default_value = "")]
    pub user: String,

    #[arg(short, long, default_value = "")]
    pub password: String,

    #[arg(short, long, default_value_t = 3)]
    pub timeout: u64,
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Subcommand)]
pub enum Commands {
    /// Create a new Keyspace inside a ScyllaDB Cluster
    Keyspace {
        #[command(flatten)]
        connection: ConnectionConfig,

        /// Keyspace name
        #[arg(short, long, default_value = "mykeyspace")]
        keyspace: String,

        /// Replication factor
        #[arg(short, long, default_value_t = 3)]
        replication_factor: u8,

        /// Drop the keyspace if it already exists
        #[arg(short, long)]
        drop: bool,

        #[arg(long)]
        without_tablets: bool,

    },
    MultiDC {
        #[command(flatten)]
        connection: ConnectionConfig,
        /// Replication factor
        #[arg(short, long, default_value_t = 3)]
        replication_factor: u8,

        #[clap(short, long, value_parser, num_args = 1.., value_delimiter = ',', default_value = "DC1,DC2")]
        dcs: Vec<String>,

        #[arg(short, long, default_value = "mykeyspace")]
        keyspace: String,
    },

}

impl fmt::Display for Commands {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Commands::Keyspace { .. } => write!(f, "New Keyspace"),
            Commands::MultiDC { .. } => { write!(f, "Multi DC Setup") }
        }
    }
}