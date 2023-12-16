use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use std::env;
use std::sync::Arc;
use std::time::Duration;

#[derive(Clone)]
pub struct Db(pub(crate) Arc<DatabaseConnection>);

impl Db {
    pub async fn new() -> Db {
        // 接続プールのオプションを設定する
        let mut options = ConnectOptions::new(
            env::var("DATABASE_URL").unwrap_or_else(|_| panic!("DATABASE_URL must be set!")),
        );
        options
            .max_connections(10) // 最大接続数
            .min_connections(5) // 最小接続数
            .connect_timeout(Duration::from_secs(10)) // 接続タイムアウト
            .idle_timeout(Duration::from_secs(10)) // アイドルタイムアウト
            .max_lifetime(Duration::from_secs(10)) // 最大生存期間
            .sqlx_logging(true);

        // 接続プールを取得して返す
        let conn = Database::connect(options).await.unwrap_or_else(|_| {
            panic!("Cannot connect to the database. Please check your configuration.")
        });

        Db(Arc::new(conn))
    }
}
