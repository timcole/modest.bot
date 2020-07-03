mod commands;
mod handler;
mod shard;
mod twitch;

use commands::{help::*, kill::*, ping::*};
use dotenv;
use serenity::client::Client;
use serenity::framework::standard::macros::group;
use serenity::framework::standard::StandardFramework;
use serenity::model::id::UserId;
use shard::ShardManagerContainer;
use std::{collections::HashSet, sync::Arc};

#[group]
#[commands(ping, help, kill)]
struct General;

fn main() {
  dotenv::dotenv();

  let token: String = dotenv::var("DISCORD_TOKEN").expect("Missing token env");

  let mut client: Client = Client::new(&token, handler::Handler).expect("Error creating client");

  {
    let mut data = client.data.write();
    data.insert::<ShardManagerContainer>(Arc::clone(&client.shard_manager));
  }

  let mut owners = HashSet::new();
  owners.insert(UserId(83281345949728768));

  client.with_framework(
    StandardFramework::new()
      .configure(|c| c.owners(owners).prefix("!"))
      .group(&GENERAL_GROUP),
  );

  if let Err(e) = client.start_shards(1) {
    println!("Client error: {:?}", e);
  }
}
