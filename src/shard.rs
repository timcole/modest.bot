use bb8_postgres::PostgresConnectionManager;
use postgres::NoTls;
use serenity::client::bridge::gateway::ShardManager;
use serenity::prelude::Mutex;
use serenity::prelude::TypeMapKey;
use std::sync::Arc;

pub struct ShardManagerContainer;

impl TypeMapKey for ShardManagerContainer {
  type Value = Arc<Mutex<ShardManager>>;
}

pub struct PostgresPool;

impl TypeMapKey for PostgresPool {
  type Value = bb8::Pool<PostgresConnectionManager<NoTls>>;
}
