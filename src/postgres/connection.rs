use bb8::{Pool, RunError};
use bb8_postgres::PostgresConnectionManager;
use std::env;
use tokio_postgres::{Config, Error, NoTls};

pub async fn setup() -> Result<Pool<PostgresConnectionManager<NoTls>>, RunError<Error>> {
  let dsn: Config = match env::var("POSTGRES_DSN")
    .expect("POSTGRES_DSN missing")
    .parse()
  {
    Ok(dsn) => dsn,
    Err(err) => panic!("{:?}", err),
  };

  let manager = PostgresConnectionManager::new(dsn, NoTls);

  let pool = match Pool::builder().build(manager).await {
    Ok(pool) => pool,
    Err(e) => panic!("builder error: {:?}", e),
  };

  // TODO: Build migrations
  Ok(pool)
}
