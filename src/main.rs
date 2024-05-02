use clap::Parser;
use colored::Colorize;

use crate::commands::Command;
use crate::commands::keyspace_command::new_keyspace;

mod connection;
mod commands;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Action to perform
    #[command(subcommand)]
    command: Option<Command>,

    /// Scylla Host
    #[arg(long, default_value = "localhost:9042")]
    host: String,

    #[arg(short, long, default_value = "")]
    user: String,

    #[arg(short, long, default_value = "")]
    password: String,

    #[arg(short, long, default_value_t = 3)]
    timeout: u64,
}


#[tokio::main]
async fn main() {
    let args = Args::parse();
    welcome(&args);

    let connection = connection::setup_connection(&args).await;


    match args.command {
        Some(Command::Keyspace { keyspace, replication_factor, drop }) => {
            println!("{} {}", "Action: ".cyan(), "New Keyspace");
            new_keyspace(connection, keyspace, replication_factor, drop).await;
        }
        _ => {
            println!("{}", "No command provided".red());
        }
    }
}

fn welcome(args: &Args) {
    // limit only the 15 characters from the host
    let host_port_stripped = args.host.split(":").collect::<Vec<&str>>()[0];
    let user = if args.user.is_empty() { "(default)" } else { &args.user };
    let password = if args.password.is_empty() { "(default)" } else { &args.password };

    println!("{}", "ScyllaDB Toolkit by @danielhe4rt".cyan());
    println!("{} {}", "Host:".cyan(), host_port_stripped.magenta());
    println!("{} {}", "User:".cyan(), user.magenta());
    println!("{} {}", "User:".cyan(), password.magenta());
}