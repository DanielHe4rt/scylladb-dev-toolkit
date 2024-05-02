use colored::Colorize;
use scylla::Session;

pub async fn new_keyspace(
    session: Session,
    keyspace: String,
    replication_factor: u8,
    drop_if_exists: bool,
) {

    if drop_if_exists {
        let query = format!("DROP KEYSPACE IF EXISTS {};", keyspace);
        println!("{}", "Dropping Keyspace...".cyan());
        println!("{}", query.yellow());
        session.query(query, []).await.unwrap();
    }

    let rf = format!("{} {} {} {}", "{", "'class': 'NetworkTopologyStrategy', 'replication_factor': ", replication_factor, "}");
    let query = format!(
        "CREATE KEYSPACE IF NOT EXISTS {} WITH \
        replication = {} AND durable_writes = true;"
        , keyspace
        , rf
    );

    println!("{}", "Creating new Keyspace...".cyan());
    println!("{}", query.yellow());

    session.query(query, []).await.unwrap();
    println!("{}", "It's ready! Don't forget to change it on your driver configs.".green());
}