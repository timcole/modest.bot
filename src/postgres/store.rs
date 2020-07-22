use crate::discord::shard::PostgresPool;
use postgres_array::Array;
use serenity::{
  client::Context,
  model::{
    channel::Message,
    guild::{Guild, Member, PartialGuild, Role},
    id::GuildId,
  },
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

pub async fn part_guild(ctx: Context, guild: PartialGuild) {
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
      "
      INSERT INTO guilds
        (id, name, owner_id, region, splash, banner, icon, features, vanity, description)
      VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
      ",
      &[
        &i64::try_from(*guild.id.as_u64()).unwrap(),
        &guild.name,
        &i64::try_from(*guild.owner_id.as_u64()).unwrap(),
        &guild.region,
        &guild.splash,
        &guild.banner,
        &guild.icon,
        &Array::from_vec(guild.features.clone(), guild.features.len() as i32),
        &guild.vanity_url_code,
        &guild.description,
      ],
    )
    .await
  {
    Ok(_) => log::debug!("Stored Guild"),
    Err(e) => log::error!("{:#?}", e),
  };
}

pub async fn guild(ctx: Context, guild: Guild) {
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
      "
      INSERT INTO guilds
        (id, name, owner_id, region, splash, banner, icon, features, vanity, description)
      VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
      ",
      &[
        &i64::try_from(*guild.id.as_u64()).unwrap(),
        &guild.name,
        &i64::try_from(*guild.owner_id.as_u64()).unwrap(),
        &guild.region,
        &guild.splash,
        &guild.banner,
        &guild.icon,
        &Array::from_vec(guild.features.clone(), guild.features.len() as i32),
        &guild.vanity_url_code,
        &guild.description,
      ],
    )
    .await
  {
    Ok(_) => log::debug!("Stored Guild"),
    Err(e) => log::error!("{:#?}", e),
  };

  for (_, r) in guild.roles.clone().iter() {
    role(ctx.clone(), guild.id, r).await;
  }
}

pub async fn role(ctx: Context, guild_id: GuildId, role: &Role) {
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
      "
        INSERT INTO roles
          (id, guild_id, name, colour, hoist, mentionable, permissions, position)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
        ON CONFLICT (id, guild_id) DO UPDATE SET
          name = EXCLUDED.name,
          colour = EXCLUDED.colour,
          hoist = EXCLUDED.hoist,
          mentionable = EXCLUDED.mentionable,
          permissions = EXCLUDED.permissions,
          position = EXCLUDED.position
        ",
      &[
        &i64::try_from(*role.id.as_u64()).unwrap(),
        &i64::try_from(*guild_id.as_u64()).unwrap(),
        &role.name,
        &role.colour.hex(),
        &role.hoist,
        &role.mentionable,
        &i64::try_from(role.permissions.bits).unwrap(),
        &i16::try_from(role.position).unwrap(),
      ],
    )
    .await
  {
    Ok(_) => log::debug!("Stored Role"),
    Err(e) => log::error!("{:#?}", e),
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
      (id, guild_id, avatar, bot, name, nick, discriminator, roles)
    VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
    ON CONFLICT (id, guild_id) DO UPDATE SET
      avatar = EXCLUDED.avatar, name = EXCLUDED.name, discriminator = EXCLUDED.discriminator, roles = EXCLUDED.roles, nick = EXCLUDED.nick";
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
          &member.nick,
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
