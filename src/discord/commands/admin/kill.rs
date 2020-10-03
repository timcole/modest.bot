use crate::discord::shard::ShardManagerContainer;
use serenity::{
  framework::standard::{macros::command, CommandResult},
  model::prelude::Message,
  prelude::Context,
};

#[command]
async fn kill(ctx: &Context, msg: &Message) -> CommandResult {
  let data = ctx.data.read().await;

  if let Some(manager) = data.get::<ShardManagerContainer>() {
    let _ = msg.reply(&ctx.http, "Goodbye :(").await;
    manager.lock().await.shutdown_all().await;
  } else {
    let _ = msg
      .reply(&ctx.http, "There was a problem getting the shard manager")
      .await;
  }

  Ok(())
}
