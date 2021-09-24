use serenity::{
  client::Context,
  model::{channel::Message, id::UserId},
};
use std::time::Duration;
use tokio::time::sleep;

pub async fn tim(ctx: &Context, msg: &Message) {
  if msg.mentions_user_id(&UserId(83281345949728768)) {
    let dead = msg
      .reply(
        &ctx.http,
        "Imagine pinging Tim... <:dead:890777799537885255>",
      )
      .await
      .ok()
      .unwrap();

    let ctx = ctx.clone();
    tokio::spawn(async move {
      sleep(Duration::from_millis(10000)).await;
      if !dead.delete(&ctx).await.is_err() {
        log::debug!("deleted ping taunt to prevent spam");
      }
    });
  }
}
