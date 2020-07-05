mod commands;
mod handler;
mod shard;
mod twitch;
mod utils;

use commands::help::*;
use commands::*;
use dotenv;
use serenity::{client::Client, framework::standard::StandardFramework, model::id::UserId};
use shard::ShardManagerContainer;
use std::{collections::HashSet, env, sync::Arc};

extern crate pretty_env_logger;

#[tokio::main]
async fn main() {
  dotenv::dotenv().ok();
  pretty_env_logger::init();

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
  }

  client
    .start_autosharded()
    .await
    .expect("Failed to start autosharding.");
}
