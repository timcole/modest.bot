use crate::twitch::automod::*;
use serenity::{
  async_trait,
  client::Context,
  model::{channel::Message, gateway::Ready, id::UserId},
  prelude::EventHandler,
};
use std::time::Duration;
use tokio::time::delay_for;

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
  async fn ready(&self, _: Context, ready: Ready) {
    if let Some(shard) = ready.shard {
      println!(
        "{} connected on shard {}/{}",
        ready.user.name,
        shard[0] + 1,
        shard[1],
      );
    }
  }
  async fn message(&self, ctx: Context, msg: Message) {
    println!(
      "({id}) {name}#{discrim}: {content}",
      id = msg.author.id,
      name = msg.author.name,
      discrim = msg.author.discriminator,
      content = msg.content
    );

    // Ignore messages from the bot
    if msg.is_own(&ctx).await {
      return;
    }

    let tim = UserId(83281345949728768).to_user(&ctx).await.unwrap();
    if msg.mentions.contains(&tim) {
      let _ = msg.channel_id.say(
        &ctx.http,
        "Imagine pinging Tim... <:haHaa:340276843523473409>",
      );

      return;
    }

    match automod(msg.content.clone()).await {
      Ok(bool) => {
        if !bool {
          match msg.delete(&ctx).await {
            Ok(_) => {}
            Err(why) => println!("{}", why),
          };
          match msg
            .reply(
              &ctx,
              "**Your message was blocked by Automod. Please watch what you say.**",
            )
            .await
          {
            Ok(msg) => {
              tokio::spawn(async move {
                delay_for(Duration::from_millis(15000)).await;
                let _ = msg.delete(&ctx).await;
              });
            }
            Err(e) => println!("{}", e),
          };
        }
      }
      Err(e) => println!("Automod Error: {}", e),
    };
  }
}
