use colored::Colorize;
use scylla::{Session, SessionBuilder};

use crate::Args;

pub async fn setup_connection(args: &Args) -> Session {
    println!("{}", "-".repeat(50));
    println!("{}", "Setting up connection...".green());
    let session = SessionBuilder::new()
        .known_nodes(vec![args.host.as_str()])
        .user(args.user.as_str(), args.password.as_str())
        .connection_timeout(std::time::Duration::from_secs(args.timeout))
        .build()
        .await
        .unwrap();

    println!("{}", "Connection established!".green());
    println!("{}", "-".repeat(50));

    session
}