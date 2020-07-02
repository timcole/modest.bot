use serenity::client::bridge::gateway::ShardManager;
use serenity::prelude::Mutex;
use serenity::prelude::TypeMapKey;
use std::sync::Arc;

pub struct ShardManagerContainer;

impl TypeMapKey for ShardManagerContainer {
  type Value = Arc<Mutex<ShardManager>>;
}
