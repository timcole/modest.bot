use crate::discord::{
  commands::{help::*, *},
  handler,
  shard::{PostgresPool, ShardManagerContainer},
};
use bb8_postgres::PostgresConnectionManager;
use postgres::NoTls;
use serenity::{client::Client, framework::standard::StandardFramework, model::id::UserId};
use std::{collections::HashSet, env, sync::Arc};

pub async fn setup(pool: &bb8::Pool<PostgresConnectionManager<NoTls>>) {
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
    .group(&ADMIN_GROUP);

  let mut client: Client = Client::builder(&token)
    .event_handler(handler::Handler)
    .framework(framework)
    .await
    .expect("Error creating client");

  {
    let mut data = client.data.write().await;
    data.insert::<ShardManagerContainer>(Arc::clone(&client.shard_manager));
    data.insert::<PostgresPool>(pool.clone());
  }

  client
    .start_autosharded()
    .await
    .expect("Failed to start autosharding.");
}
