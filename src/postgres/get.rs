use crate::discord::shard::PostgresPool;
use serenity::{
  client::Context,
  model::id::{GuildId, UserId},
};
use std::convert::TryFrom;

pub async fn is_new_guild(ctx: &Context, guild_id: GuildId) -> bool {
  let data = ctx.data.read().await;
  let pool = match data.get::<PostgresPool>() {
    Some(v) => v.get().await.unwrap(),
    None => {
      log::error!("Error getting the postgres pool.");
      return false;
    }
  };

  let has_guild = pool
    .query(
      "SELECT 1 FROM guilds WHERE id = $1",
      &[&i64::try_from(*guild_id.as_u64()).unwrap()],
    )
    .await
    .ok();

  has_guild.unwrap().get(0).is_none()
}

pub async fn strikes(ctx: &Context, guild_id: &GuildId, author_id: &UserId) -> i64 {
  let data = ctx.data.read().await;
  let pool = match data.get::<PostgresPool>() {
    Some(v) => v.get().await.unwrap(),
    None => {
      log::error!("Error getting the postgres pool.");
      return 0;
    }
  };

  let has_guild = match pool
    .query(
      "SELECT COUNT(id) FROM messages WHERE author_id = $2 AND guild_id = $1 AND permitted IS FALSE AND created_at >= NOW() - INTERVAL '15 minutes'",
      &[&i64::try_from(*guild_id.as_u64()).unwrap(), &i64::try_from(*author_id.as_u64()).unwrap()],
    )
    .await
    .ok() {
        Some(row) => row,
        None => return 0
    };

  match has_guild[0].get(0) {
    Some(row) => row,
    None => 0,
  }
}
