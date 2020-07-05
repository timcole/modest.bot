mod handler;
mod twitch;
mod utils;

use dotenv;
use serenity::client::Client;
use serenity::framework::standard::StandardFramework;
use serenity::model::id::UserId;
use std::{collections::HashSet, env};

extern crate pretty_env_logger;

#[tokio::main]
async fn main() {
  dotenv::dotenv().ok();
  pretty_env_logger::init();

  let token: String = env::var("DISCORD_TOKEN").expect("Missing token env");

  let mut owners = HashSet::new();
  owners.insert(UserId(83281345949728768));

  let mut client: Client = Client::new(&token)
    .event_handler(handler::Handler)
    .framework(StandardFramework::new().configure(|c| c.owners(owners).prefix("!")))
    .await
    .expect("Error creating client");

  client
    .start_autosharded()
    .await
    .expect("Failed to start autosharding.");
}
