use crate::twitch::automod;
use crate::utils::mention;
use serenity::{
  async_trait,
  client::Context,
  model::{channel::Message, gateway::Ready},
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
    let guild = match msg.guild_id {
      Some(id) => id,
      // Ignore DMs
      None => return,
    };

    let channel = msg.channel_id;

    log::info!(
      "[{guild}|{channel}] - ({id}) {name}#{discrim}: {content}",
      guild = guild,
      channel = channel,
      id = msg.author.id,
      name = msg.author.name,
      discrim = msg.author.discriminator,
      content = msg.content
    );

    // Ignore messages from bots
    if msg.author.bot {
      return;
    }

    if automod::automod(ctx.clone(), msg.clone())
      .await
      .ok()
      .unwrap()
    {
      mention::tim(ctx.clone(), msg.clone()).await;
    }
  }
}
