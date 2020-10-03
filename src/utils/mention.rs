use serenity::{
  client::Context,
  model::{channel::Message, id::UserId},
};

pub async fn tim(ctx: &Context, msg: &Message) {
  if msg.mentions_user_id(&UserId(83281345949728768)) {
    msg
      .reply(
        &ctx.http,
        "Imagine pinging Tim... <:haHaa:340276843523473409>",
      )
      .await
      .ok();
  }
}
