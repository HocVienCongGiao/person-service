mod config;
mod db_column;
pub mod person_gateway;
pub mod personal_id_number_gateway;
pub mod saint_gateway;

use tokio_postgres::{Client, NoTls};

pub async fn connect() -> Client {
    let config = config::Config::new();
    println!("Connecting with config {:?}", config);
    let result = tokio_postgres::connect(
        format!(
            "user={} password={} host={} port={} dbname={}",
            config.db_user, config.db_password, config.db_host, config.db_port, config.db_name,
        )
        .as_str(),
        NoTls,
    )
    .await;

    let (client, connection) = result.unwrap();
    // The connection object performs the actual communication with the database,
    // so spawn it off to run on its own.
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });
    client
}
