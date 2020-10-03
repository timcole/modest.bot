use crate::discord::shard::PostgresPool;
use serenity::{
  client::Context,
  model::id::{GuildId, RoleId, UserId},
};
use std::convert::TryFrom;

pub async fn role(ctx: &Context, guild_id: GuildId, role_id: RoleId) {
  let data = ctx.data.read().await;
  let pool = match data.get::<PostgresPool>() {
    Some(v) => v.get().await.unwrap(),
    None => {
      log::error!("Error getting the postgres pool.");
      return;
    }
  };

  match pool
    .query(
      "DELETE FROM roles WHERE id = $1 AND guild_id = $2",
      &[
        &i64::try_from(*role_id.as_u64()).unwrap(),
        &i64::try_from(*guild_id.as_u64()).unwrap(),
      ],
    )
    .await
  {
    Ok(_) => log::debug!("Deleted role {} in {}", role_id, guild_id),
    Err(e) => log::error!("{:#?}", e),
  };
}

pub async fn member(ctx: Context, guild_id: GuildId, user_id: UserId) {
  let data = ctx.data.read().await;
  let pool = match data.get::<PostgresPool>() {
    Some(v) => v.get().await.unwrap(),
    None => {
      log::error!("Error getting the postgres pool.");
      return;
    }
  };

  match pool
    .query(
      "DELETE FROM members WHERE id = $1 AND guild_id = $2",
      &[
        &i64::try_from(*user_id.as_u64()).unwrap(),
        &i64::try_from(*guild_id.as_u64()).unwrap(),
      ],
    )
    .await
  {
    Ok(_) => log::debug!("Deleted member {} in {}", user_id, guild_id),
    Err(e) => log::error!("{:#?}", e),
  };
}
