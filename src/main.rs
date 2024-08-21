use clap::Parser;
use colored::Colorize;

use crate::commands::{Commands, keyspace_command, setup_multi_dc_command};
use crate::commands::table_actions_command;

mod connection;
mod commands;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Action to perform
    #[command(subcommand)]
    command: Option<Commands>,
}


#[tokio::main]
async fn main() -> anyhow::Result<()> {
    welcome();
    let args = Args::parse();

    match args.command {
        Some(Commands::Keyspace { keyspace, replication_factor, drop, without_tablets, connection }) => {
            let connection = connection::setup_connection(&connection).await;
            println!("{} {}", "Action: ".cyan(), "New Keyspace");
            keyspace_command::handle(connection, keyspace, replication_factor, without_tablets, drop).await;
        },
        Some(Commands::MultiDC { replication_factor, dcs, keyspace, connection}) => {
            let connection = connection::setup_connection(&connection).await;
            println!("{} {}", "Action: ".cyan(), "Multi DC Setup");
            setup_multi_dc_command::handle(connection, keyspace, dcs, replication_factor).await;
        },
        Some(Commands::TableActions { connection, keyspace, suffix, action }) => {
            let connection = connection::setup_connection(&connection).await;
            println!("{} {}", "Action: ".cyan(), "Table Actions");
            table_actions_command::handle(connection, keyspace, action, suffix).await?;
        },
        _ => {
            println!("{}", "No command provided".red());
        }
    }

    Ok(())
}

fn welcome() {
    println!("{}", "ScyllaDB Toolkit by @danielhe4rt".cyan());
}