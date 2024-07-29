use colored::Colorize;
use scylla::Session;

pub async fn handle(
    session: Session,
    keyspace: String,
    replication_factor: u8,
    without_tablets: bool,
    drop_if_exists: bool,
) {
    if drop_if_exists {
        let query = format!("DROP KEYSPACE IF EXISTS {};", keyspace);
        println!("{}", "Dropping Keyspace...".cyan());
        println!("{}", query.yellow());
        session.query(query, []).await.unwrap();
    }

    let has_tablets = if without_tablets {
        "AND tablets = {'enabled': false}"
    } else {
        "AND tablets = {'enabled': true}"
    };

    let rf = format!("{} {} {} {}", "{", "'class': 'NetworkTopologyStrategy', 'replication_factor': ", replication_factor, "}");
    let query = format!(
        "CREATE KEYSPACE IF NOT EXISTS {} WITH replication = {} AND durable_writes = true {}",
        keyspace,
        rf,
        has_tablets
    );

    println!("{}", "Creating new Keyspace...".cyan());
    println!("{}", query.yellow());

    session.query(query, []).await.unwrap();
    println!("{}", "It's ready! Don't forget to change it on your driver configs.".green());
}