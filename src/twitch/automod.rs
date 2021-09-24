use serde::{Deserialize, Serialize};
use serenity::{client::Context, model::channel::Message};
use std::env;
use std::time::Duration;
use tokio::time::sleep;

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

const AUTOMOD_GUILD_MESSAGE: &'static str =
  "**Your message was blocked by Automod. Please watch what you say.**
_If you had DM's turned on, I would have sent your message back to you to fix your mistake._";

pub async fn automod(ctx: &Context, msg: &Message) -> Result<bool, reqwest::Error> {
  let client_id: String = env::var("TWITCH_CLIENT").expect("Missing twitch client");
  let broadcaster_id: String = env::var("TWITCH_BROADCASTER_ID").unwrap_or("51684790".to_string());
  let bearer: String = format!(
    "Bearer {}",
    dotenv::var("TWITCH_BEARER").expect("Missing twitch bearer")
  );

  let client = reqwest::Client::new();
  let automod_resp: Resp = client
    .post("https://api.twitch.tv/helix/moderation/enforcements/status")
    .query(&[("broadcaster_id", broadcaster_id)])
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

  let (safe_content, guild) = tokio::join!(msg.content_safe(&ctx.cache), msg.guild(&ctx.cache));
  let dm_warning = msg
    .author
    .direct_message(&ctx, |m| {
      m.content(format!(
        "**Your message in {guild_name} (<#{channel_id}>) was blocked by Automod. Please watch what you say next time.**\n```{message_safe}```",
        guild_name = &guild.unwrap().name,
        channel_id = msg.channel_id,
        message_safe = &safe_content
      ))
    })
    .await;

  match dm_warning.is_err() {
    false => log::info!("yelled at {} in DM's", msg.author),
    true => {
      let warning = msg
        .reply_ping(&ctx, AUTOMOD_GUILD_MESSAGE)
        .await
        .ok()
        .unwrap();
      let ctx = ctx.clone();
      tokio::spawn(async move {
        sleep(Duration::from_millis(15000)).await;
        if !warning.delete(&ctx).await.is_err() {
          log::debug!("deleted warning to prevent spam");
        }
      });
    }
  };

  if msg.delete(ctx).await.is_err() {
    log::error!("failed to delete message");
    return Ok(false);
  }

  Ok(false)
}
