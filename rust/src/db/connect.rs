use crate::config::config::DatabaseConfig;
use tokio_postgres::{Client, Error, NoTls};

pub async fn connect(db_config: &DatabaseConfig) -> Result<Client, Error> {
    let (client, connection) = tokio_postgres::connect(
        &format!(
            "host={} user={} password={} port={} dbname={}",
            db_config.host, db_config.user, db_config.password, db_config.port, db_config.dbname
        )
        .to_string(),
        NoTls,
    )
    .await?;
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });
    Ok(client)
}
