use crate::PostgresPool;
use cast::i64;
use serenity::{client::Context, model::channel::Message};

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
        &i64(*msg.id.as_u64()).unwrap(),
        &i64(*match msg.guild_id { Some(id) => id, None => return }.as_u64()).unwrap(),
        &i64(*msg.channel_id.as_u64()).unwrap(),
        &i64(*msg.author.id.as_u64()).unwrap(),
        &msg.content,
        &permitted
      ]
    )
    .await {
        Ok(_) => log::debug!("Stored message"),
        Err(e) => log::error!("{:#?}", e)
    };
}
