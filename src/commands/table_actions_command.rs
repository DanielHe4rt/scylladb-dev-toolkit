use scylla::transport::topology::Keyspace;
use scylla::Session;
use std::io;

pub async fn handle(
    session: Session,
    keyspace: String,
    action: Option<u8>,
    suffix: Option<String>,
) -> anyhow::Result<()> {
    let mut buffer = String::new();
    let stdin = io::stdin();
    let cluster_data = session.get_cluster_data();
    let keyspace_info = cluster_data.get_keyspace_info();

    let keyspace_specs = keyspace_info.get(&keyspace).expect("Keyspace not found");

    if action.is_some() || suffix.is_some() {
        execute_actions(session, keyspace, keyspace_specs, action.unwrap(), suffix.unwrap().as_str()).await?;
        return Ok(());
    }

    list_actions(keyspace.clone(), keyspace_specs);
    stdin.read_line(&mut buffer)?;

    let choice = buffer.trim().parse::<u8>()?;

    println!("Select a Suffix for the tables which will be targeted:");

    buffer.clear();
    stdin.read_line(&mut buffer)?;

    let suffix = buffer.trim();

    println!("Selected Suffix: {:?}", suffix);
    execute_actions(session, keyspace, keyspace_specs, choice, suffix).await?;

    Ok(())
}

fn list_actions(keyspace: String, keyspace_specs: &Keyspace) {
    println!("Keyspace: {}", keyspace);
    for table in keyspace_specs.tables.iter() {
        println!("  - T: {}", table.0);
    }
    for table in keyspace_specs.views.iter() {
        println!("  - MV: {}", table.0);
    }

    println!("What do you want to do?");
    println!(" > 1. Truncate Tables");
    println!(" > 2. Drop Tables");
}

async fn execute_actions(session: Session, keyspace: String, keyspace_specs: &Keyspace, choice: u8, suffix: &str) -> anyhow::Result<()> {
    let mut tables = vec![];
    keyspace_specs.views.iter().for_each(|(table_name, _)| {
        tables.push(("MATERIALIZED VIEW", table_name));
    });

    keyspace_specs.tables.iter().for_each(|(table_name, _)| {
        tables.push(("TABLE", table_name));
    });

    match choice {
        1 => {
            for (_, table_name) in tables {
                if table_name.ends_with(suffix) {
                    let query = format!("TRUNCATE {}.{};", keyspace, table_name);
                    println!(" > Truncating table: {}", table_name);
                    session.query(query, []).await?;
                } else {
                    println!(" x Skipping Table table: {}", table_name);
                }
            }
        }
        2 => {
            for (operation, table_name) in tables {
                if table_name.ends_with(suffix) {
                    let query = format!("DROP {} {}.{};", operation, keyspace, table_name);
                    println!(" > Dropping: {}", table_name);
                    session.query(query, []).await?;
                } else {
                    println!(" x Skipping: {}", table_name);
                }
            }
        }
        _ => {
            println!("Invalid choice");
        }
    }
    Ok(())
}