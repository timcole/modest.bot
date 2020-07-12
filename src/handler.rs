use crate::postgres::store;
use crate::twitch::automod;
use crate::utils::mention;
use serenity::{
  async_trait,
  client::Context,
  model::{
    channel::Message,
    gateway::Activity,
    gateway::Ready,
    guild::{Guild, Member, PartialGuild, Role},
    id::{GuildId, RoleId, UserId},
    user::{OnlineStatus, User},
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
      "{channel_id} > {message_id} > {name}#{discrim} ({user_id}): {content}",
      channel_id = msg.channel_id,
      message_id = msg.id,
      user_id = msg.author.id,
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
  async fn guild_create(&self, ctx: Context, guild: Guild, is_new: bool) {
    if !is_new && !store::is_new_guild(ctx.clone(), guild.id.clone()).await {
      return;
    }

    let target = &format!("processing::{}", guild.id)[..];
    log::info!(target: target, "{}", guild.id);

    store::guild(ctx.clone(), guild.clone()).await;

    let mut after: Option<UserId> = Some(UserId(0));
    while after.is_some() {
      let members = match guild.members(&ctx.http, Some(1000), after).await {
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
  async fn guild_role_create(&self, ctx: Context, guild_id: GuildId, new: Role) {
    store::role(ctx, guild_id, &new).await;
  }
  async fn guild_role_delete(
    &self,
    ctx: Context,
    guild_id: GuildId,
    role_id: RoleId,
    _: Option<Role>,
  ) {
    store::del_role(ctx, guild_id, role_id).await;
  }
  async fn guild_role_update(&self, ctx: Context, guild_id: GuildId, _: Option<Role>, new: Role) {
    store::role(ctx, guild_id, &new).await;
  }
  async fn guild_update(&self, ctx: Context, _: Option<Guild>, guild: PartialGuild) {
    store::part_guild(ctx, guild).await;
  }
  async fn guild_member_addition(&self, ctx: Context, _: GuildId, member: Member) {
    store::members(ctx, vec![member]).await;
  }
  async fn guild_member_update(&self, ctx: Context, _: Option<Member>, member: Member) {
    store::members(ctx, vec![member]).await;
  }
  async fn guild_member_removal(
    &self,
    ctx: Context,
    guild_id: GuildId,
    user: User,
    _: Option<Member>,
  ) {
    store::del_member(ctx, guild_id, user.id).await;
  }
}
