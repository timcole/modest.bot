use crate::PostgresPool;
use postgres_array::Array;
use serenity::{
  client::Context,
  model::{channel::Message, guild::Member},
};
use std::convert::TryFrom;

pub async fn message(ctx: Context, msg: Message, permitted: bool) {
  let data = ctx.data.read().await;
  let pool = match data.get::<PostgresPool>() {
    Some(v) => v.get().await.unwrap(),
    None => {
      log::error!("Error getting the postgres pool.");
      return;
    }
  };

  match pool
    .execute(
      "INSERT INTO messages (id, guild_id, channel_id, author_id, content, permitted) VALUES ($1, $2, $3, $4, $5, $6)",
      &[
        &i64::try_from(*msg.id.as_u64()).unwrap(),
        &i64::try_from(*match msg.guild_id { Some(id) => id, None => return }.as_u64()).unwrap(),
        &i64::try_from(*msg.channel_id.as_u64()).unwrap(),
        &i64::try_from(*msg.author.id.as_u64()).unwrap(),
        &msg.content,
        &permitted
      ]
    )
    .await {
        Ok(_) => log::debug!("Stored message"),
        Err(e) => log::error!("{:#?}", e)
    };
}

pub async fn members(ctx: Context, mut members: Vec<Member>) {
  let data = ctx.data.read().await;
  let pool = match data.get::<PostgresPool>() {
    Some(v) => v.get().await.unwrap(),
    None => {
      log::error!("Error getting the postgres pool.");
      return;
    }
  };
  let query = "
    INSERT INTO members
      (id, guild_id, avatar, bot, name, discriminator, roles)
    VALUES ($1, $2, $3, $4, $5, $6, $7)
    ON CONFLICT (id, guild_id) DO UPDATE SET
      avatar = EXCLUDED.avatar, name = EXCLUDED.name, discriminator = EXCLUDED.discriminator, roles = EXCLUDED.roles";
  while !&members.is_empty() {
    let member = members.pop().unwrap();
    // TODO: Stop with the single inserts and move to grouping inserts.
    match pool
      .execute(
        query,
        &[
          &i64::try_from(*member.user.id.as_u64()).unwrap(),
          &i64::try_from(*member.guild_id.as_u64()).unwrap(),
          &member.user.avatar,
          &member.user.bot,
          &member.user.name,
          &i16::try_from(member.user.discriminator).unwrap(),
          &Array::from_vec(
            member
              .roles
              .iter()
              .map(|r| i64::try_from(*r.as_u64()).unwrap())
              .collect::<Vec<i64>>(),
            member.roles.len() as i32,
          ),
        ],
      )
      .await
    {
      Ok(_) => log::debug!("Stored member {} of {}", member.user.name, member.guild_id),
      Err(e) => log::error!("{:#?}", e),
    };
  }
}
