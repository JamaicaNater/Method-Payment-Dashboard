use envconfig::Envconfig;
use sqlx::mysql::MySqlConnectOptions;
use sqlx::{MySql, MySqlPool, Pool};

pub struct DBClient {
    pub(crate) pool: Option<Pool<MySql>>,
}

#[derive(Envconfig)]
struct DBConfig {
    #[envconfig(from = "DB_HOST")]
    host: String,
    #[envconfig(from = "DB_NAME")]
    database: String,
    #[envconfig(from = "DB_USER")]
    username: String,
    #[envconfig(from = "DB_PASS")]
    password: String,
    #[envconfig(from = "DB_PORT", default = "3306")]
    port: u16,
}

impl DBClient {
    fn new() -> Self {
        DBClient { pool: None }
    }
    async fn init(&mut self) -> Result<(), sqlx::Error> {
        let db_config: DBConfig = DBConfig::init_from_env().unwrap();
        let options = MySqlConnectOptions::new()
            .database(db_config.database.as_str())
            .host(db_config.host.as_str())
            .username(db_config.username.as_str())
            .password(db_config.password.as_str())
            .port(db_config.port);

        self.pool = Option::from(MySqlPool::connect_with(options).await?);
        Ok(())
    }
}

pub async fn create_from_env() -> Result<DBClient, sqlx::Error> {
    let mut client = DBClient::new();
    client.init().await?;
    Ok(client)
}
