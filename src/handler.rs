use crate::twitch::automod;
use serenity::{
  async_trait,
  client::Context,
  model::{channel::Message, gateway::Ready, id::UserId},
  prelude::EventHandler,
};

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
  async fn ready(&self, _: Context, ready: Ready) {
    if let Some(shard) = ready.shard {
      log::info!(
        "{} connected on shard {}/{}",
        ready.user.name,
        shard[0] + 1,
        shard[1],
      );
    }
  }
  async fn message(&self, ctx: Context, msg: Message) {
    log::info!(
      "({id}) {name}#{discrim}: {content}",
      id = msg.author.id,
      name = msg.author.name,
      discrim = msg.author.discriminator,
      content = msg.content
    );

    // Ignore messages from the bot
    if msg.is_own(&ctx).await {
      return;
    }

    let tim = UserId(83281345949728768).to_user(&ctx).await.unwrap();
    if msg.mentions.contains(&tim) {
      let _ = msg.channel_id.say(
        &ctx.http,
        "Imagine pinging Tim... <:haHaa:340276843523473409>",
      );

      return;
    }

    automod::automod(ctx, msg).await;
  }
}
