use anyhow::{Ok, Result};
use rocket::tokio::{self, runtime::Runtime};
use tokio_postgres::{tls::NoTlsStream, NoTls, Socket};

pub struct PostgresHelper {
    pub client: tokio_postgres::Client,
    pub connection: tokio_postgres::Connection<Socket, NoTlsStream>,
}
impl PostgresHelper {
    pub fn new(config: &str) -> Result<Self> {
        let rt = Runtime::new().unwrap();
        let (client, connection) = rt
            .block_on(async { tokio_postgres::connect(config, NoTls).await })
            .unwrap();
        Ok(Self { client, connection })
    }
    pub async fn insert_account(&mut self, user: &str, password: &str) -> Result<()> {
        Ok(())
    }
}
#[tokio::test]
async fn test() {
    let db =
        PostgresHelper::new("host=152.136.130.225 user=app_user password='1222abcABC' dbname=app")
            .await
            .unwrap();
    db.insert_account("fd", "fs").await.unwrap();
}
