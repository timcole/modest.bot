use crate::PostgresPool;
use regex::Regex;
use serenity::{
  framework::standard::{macros::command, Args, CommandResult},
  model::prelude::Message,
  prelude::Context,
};

#[command]
async fn sql(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
  let data = ctx.data.read().await;
  let pool = match data.get::<PostgresPool>() {
    Some(v) => v.get().await.unwrap(),
    None => {
      log::error!("Error getting the postgres pool.");
      return Ok(());
    }
  };

  let re = Regex::new(r"(```(sql)?)").unwrap();
  let query = re.replace_all(args.message(), "");

  let guilds = pool
    .query(
      &format!(
        "SELECT row_to_json(t, false)::TEXT FROM ({} LIMIT 10) t",
        query.trim()
      )[..],
      &[],
    )
    .await?;
  let mut resp = String::new();
  for guild in guilds {
    let line: &str = guild.get(0);
    resp.push_str(&format!("{}\n", line)[..]);
  }

  msg
    .channel_id
    .say(&ctx.http, format!("```json\n{}\n```", resp))
    .await
    .ok();

  Ok(())
}
