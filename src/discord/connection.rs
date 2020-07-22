use crate::discord::{
  commands::{help::*, *},
  handler,
  shard::{PostgresPool, ShardManagerContainer},
};
use bb8_postgres::PostgresConnectionManager;
use postgres::NoTls;
use serenity::client::bridge::gateway::ShardManager;
use serenity::prelude::Mutex;
use serenity::{client::Client, framework::standard::StandardFramework, model::id::UserId};
use std::{collections::HashSet, env, sync::Arc};

pub async fn setup(pool: bb8::Pool<PostgresConnectionManager<NoTls>>) -> Arc<Mutex<ShardManager>> {
  let token: String = env::var("DISCORD_TOKEN").expect("Missing token env");

  let mut owners = HashSet::new();
  owners.insert(UserId(83281345949728768));

  let framework = StandardFramework::new()
    .configure(|c| {
      c.owners(owners)
        .prefix("!")
        .ignore_bots(true)
        .ignore_webhooks(true)
        .allow_dm(false)
    })
    .help(&HELP)
    // .group(&GENERAL_GROUP)
    .group(&ADMIN_GROUP);

  let mut client: Client = Client::new(&token)
    .event_handler(handler::Handler)
    .framework(framework)
    .await
    .expect("Error creating client");

  {
    let mut data = client.data.write().await;
    data.insert::<ShardManagerContainer>(Arc::clone(&client.shard_manager));
    data.insert::<PostgresPool>(pool.clone());
  }

  let shards = Arc::clone(&client.shard_manager);
  tokio::spawn(async move {
    client
      .start_autosharded()
      .await
      .expect("Failed to start autosharding.");
  });
  shards
}
