use crate::discord::shard::PostgresPool;
use serenity::{client::Context, model::id::GuildId};
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
