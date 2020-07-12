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

  let conn = pool.clone();
  let conn = conn.get().await.unwrap();
  match conn
    .batch_execute(
      "CREATE TABLE IF NOT EXISTS public.guilds (
        id int8 NOT NULL,
        name text,
        owner_id int8,
        region varchar(32),
        splash varchar(255),
        banner varchar(255),
        icon varchar(255),
        features _varchar DEFAULT '{}'::character varying[],
        vanity varchar(255),
        description text,
        PRIMARY KEY (id)
      );

      CREATE TABLE IF NOT EXISTS public.members (
        id int8 NOT NULL,
        guild_id int8 NOT NULL,
        avatar varchar(255),
        bot bool,
        name varchar(255),
        discriminator int2,
        roles _int8 NOT NULL DEFAULT '{}'::bigint[],
        nick varchar(255),
        PRIMARY KEY (id,guild_id)
      );

      CREATE TABLE IF NOT EXISTS public.messages (
        id int8 NOT NULL,
        guild_id int8 NOT NULL,
        channel_id int8 NOT NULL,
        author_id int8 NOT NULL,
        content text NOT NULL,
        permitted bool NOT NULL,
        PRIMARY KEY (id,guild_id,channel_id)
      );

      CREATE TABLE IF NOT EXISTS public.roles (
        id int8 NOT NULL,
        colour varchar(6),
        hoist bool,
        mentionable bool,
        name varchar(255),
        permissions int8,
        position int2,
        guild_id int8 NOT NULL,
        PRIMARY KEY (id,guild_id)
      );

      ALTER TABLE public.members ADD FOREIGN KEY (guild_id) REFERENCES public.guilds(id);
      ALTER TABLE public.messages ADD FOREIGN KEY (guild_id) REFERENCES public.guilds(id);",
    )
    .await
  {
    Ok(_) => log::debug!("Ran Migrations"),
    Err(e) => panic!("Failed to run migrations: {:?}", e),
  };

  Ok(pool)
}
