use serenity::client::Context;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;
use serenity::model::prelude::Message;

#[command]
async pub fn ping(ctx: &mut Context, msg: &Message) -> CommandResult {
  let latency: i64 =
    chrono::offset::Utc::now().timestamp_millis() - msg.timestamp.timestamp_millis();

  let _ = msg.channel_id.say(&ctx.http, format!("{}ms", latency));

  return Ok(());
}
