mod commands;
mod handler;
mod postgres;
mod shard;
mod twitch;
mod utils;

use crate::postgres::connection;
use commands::help::*;
use commands::*;
use dotenv;
use serenity::{client::Client, framework::standard::StandardFramework, model::id::UserId};
use shard::{PostgresPool, ShardManagerContainer};
use std::{collections::HashSet, env, sync::Arc};

#[tokio::main]
async fn main() {
  dotenv::dotenv().ok();
  pretty_env_logger::init();

  let pool = connection::setup().await.expect("Failed to setup postgres");

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

  client
    .start_autosharded()
    .await
    .expect("Failed to start autosharding.");
}
