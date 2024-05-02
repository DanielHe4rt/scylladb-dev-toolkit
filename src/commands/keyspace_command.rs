use colored::Colorize;
use scylla::Session;

pub async fn new_keyspace(session: Session, keyspace_name: &str)
{
    let rf = "{'class': 'NetworkTopologyStrategy'}";
    let query = format!(
        "CREATE KEYSPACE IF NOT EXISTS {} WITH \
        replication = {} AND durable_writes = true;"
        , keyspace_name
        , rf
    );

    println!("{}", "Creating new Keyspace...".cyan());
    println!("{}", query.yellow());

    session.query(query, []).await.unwrap();
    println!("{}", "It's ready! Don't forget to change it on your driver configs.".green());
}