use clap::Parser;
use colored::Colorize;

use crate::commands::keyspace_command::new_keyspace;

mod connection;
mod commands;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(short, long, default_value = "")]
    action: String,

    /// Scylla Host
    #[arg(long, default_value = "localhost:9042")]
    host: String,

    /// Keyspace
    #[arg(short, long, default_value = "mykeyspace")]
    keyspace: String,

    #[arg(short, long, default_value = "")]
    user: String,

    #[arg(short, long, default_value = "")]
    password: String,

    #[arg(short, long, default_value_t = 30)]
    timeout: u64,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    welcome(&args);

    let connection = connection::setup_connection(&args).await;
    let action = args.action.as_str();

    println!("{} {}", "Action: ".cyan(), action.magenta());
    match action {
        "keyspace" => new_keyspace(connection, &args.keyspace).await,
        _ => {
            println!("Invalid action");
            std::process::exit(1);
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
    println!("{} {}", "Keyspace:".cyan(), &args.keyspace.magenta());
}