use serde::{Deserialize, Serialize};
use serenity::{client::Context, model::channel::Message};
use std::env;
use std::time::Duration;
use tokio::time::delay_for;

#[derive(Debug, Serialize, Deserialize)]
struct Data {
  msg_id: String,
  user_id: String,
  msg_text: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Body {
  data: Vec<Data>,
}

#[derive(Debug, Serialize, Deserialize)]
struct RespData {
  msg_id: String,
  is_permitted: bool,
}

#[derive(Debug, Serialize, Deserialize)]
struct Resp {
  data: Vec<RespData>,
}

const AUTOMOD_MESSAGE: &'static str =
  "**Your message was blocked by Automod. Please watch what you say.**";

pub async fn automod(ctx: &Context, msg: &Message) -> Result<bool, reqwest::Error> {
  let client_id: String = env::var("TWITCH_CLIENT").expect("Missing twitch client");
  let bearer: String = format!(
    "Bearer {}",
    dotenv::var("TWITCH_BEARER").expect("Missing twitch bearer")
  );

  let client = reqwest::Client::new();
  let automod_resp: Resp = client
    .post("https://api.twitch.tv/helix/moderation/enforcements/status?broadcaster_id=51684790")
    .json(&Body {
      data: vec![Data {
        msg_id: msg.id.as_ref().to_string(),
        user_id: String::from("1"),
        msg_text: msg.content.to_string(),
      }],
    })
    .header("Client-ID", client_id)
    .header("Authorization", bearer)
    .send()
    .await?
    .json()
    .await?;

  if automod_resp.data[0].is_permitted {
    return Ok(true);
  }

  let warning = msg.reply_ping(ctx, AUTOMOD_MESSAGE).await.ok().unwrap();
  if msg.delete(ctx).await.is_err() {
    log::error!("failed to delete message");
    return Ok(false);
  }

  let ctx = ctx.clone();
  tokio::spawn(async move {
    delay_for(Duration::from_millis(15000)).await;
    if !warning.delete(&ctx).await.is_err() {
      log::debug!("deleted warning to prevent spam");
    }
  });

  Ok(false)
}
