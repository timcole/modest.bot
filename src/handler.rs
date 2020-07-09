use crate::postgres::store;
use crate::twitch::automod;
use crate::utils::mention;
use serenity::{
  async_trait,
  client::Context,
  model::{
    channel::Message, gateway::Activity, gateway::Ready, guild::Guild, id::UserId,
    user::OnlineStatus,
  },
  prelude::EventHandler,
};
use std::env;

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
  async fn ready(&self, ctx: Context, ready: Ready) {
    if let Some(shard) = ready.shard {
      log::info!(
        "{} connected on shard {}/{}",
        ready.user.name,
        shard[0] + 1,
        shard[1],
      );
    }
    ctx
      .set_presence(
        Some(Activity::listening(&format!(
          "Version: {}",
          &env!("GIT_HASH")[0..7]
        ))),
        OnlineStatus::DoNotDisturb,
      )
      .await;
  }
  async fn message(&self, ctx: Context, msg: Message) {
    let guild = match msg.guild(&ctx).await {
      Some(guild) => guild,
      None => return,
    };

    log::info!(
      target: &format!("{} ({})", guild.id, guild.name)[..],
      "{channel} > {name}#{discrim} ({id}): {content}",
      channel = msg.channel_id,
      id = msg.author.id,
      name = msg.author.name,
      discrim = msg.author.discriminator,
      content = msg.content
    );

    // Ignore messages from bots
    if msg.author.bot {
      return;
    }

    let permitted = automod::automod(ctx.clone(), msg.clone())
      .await
      .ok()
      .unwrap();

    store::message(ctx.clone(), msg.clone(), permitted).await;

    if permitted {
      mention::tim(ctx.clone(), msg.clone()).await;
    }
  }
  async fn guild_create(&self, ctx: Context, guild: Guild, _is_new: bool) {
    let target = &format!("processing::{}", guild.id)[..];
    log::info!(target: target, "{}", guild.id);
    // TODO: uncomment
    // if !is_new {
    //   return;
    // }

    let mut after: Option<UserId> = Some(UserId(0));
    while after.is_some() {
      let members = match guild.members(&ctx.http, Some(3), after).await {
        Ok(members) => members,
        Err(_) => break,
      };

      after = match members.last() {
        Some(member) => Some(member.user.id),
        None => None,
      };
      store::members(ctx.clone(), members.clone()).await;
    }
  }
  // TODO: add hander for member join and leave
}
