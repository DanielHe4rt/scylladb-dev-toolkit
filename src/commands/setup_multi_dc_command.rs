use colored::Colorize;
use scylla::Session;

pub async fn handle(
    session: Session,
    dcs: Vec<String>,
    replication_factor: u8,
) {
    let mut rf = String::new();
    rf.push_str("{ 'class' : 'NetworkTopologyStrategy',");
    for dc in dcs {
        let dc = format!("'{}' : {},", dc, replication_factor);
        rf.push_str(dc.as_str());
    }
    rf.pop();
    rf.push_str("}");

    let queries = vec![
        ("system_auth", "ALTER KEYSPACE system_auth WITH replication = "),
        ("system_distributed", "ALTER KEYSPACE system_distributed WITH replication = "),
        ("system_traces", "ALTER KEYSPACE system_traces WITH replication = "),
    ];

    for (table, query) in queries {
        let query = format!("{}{};", query, rf);
        println!("{} {}", "Altering table:".yellow(), table);
        let prepared = session.prepare(query).await.unwrap();

        session.execute(&prepared, ()).await.unwrap();
    }
    println!("{}", "It's ready! Don't forget to change it on your driver configs.".green());
}