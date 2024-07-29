use colored::Colorize;
use scylla::{Session, SessionBuilder};

use crate::commands::ConnectionConfig;

pub async fn setup_connection(args: &ConnectionConfig) -> Session {
    println!("{}", "-".repeat(50));
    println!("{}", "Setting up connection...".green());
    let session = SessionBuilder::new()
        .known_nodes(args.host.as_str().split(',').collect::<Vec<&str>>())
        .user(args.user.as_str(), args.password.as_str())
        .connection_timeout(std::time::Duration::from_secs(args.timeout))
        .build()
        .await
        .unwrap();

    println!("{}", "Connection established!".green());
    println!("{}", "-".repeat(50));

    session
}