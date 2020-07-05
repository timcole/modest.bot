use serenity::client::Context;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;
use serenity::model::prelude::Message;

#[command]
async fn help(ctx: &mut Context, msg: &Message) -> CommandResult {
  let _ = msg.channel_id.say(&ctx.http, "Go to bed");
  Ok(())
}
