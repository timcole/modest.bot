use crate::twitch::automod::*;
use serenity::client::Context;
use serenity::model::{channel::Message, gateway::Ready, id::UserId};
use serenity::prelude::EventHandler;
use std::thread;
use std::time::Duration;

pub struct Handler;

impl EventHandler for Handler {
  fn ready(&self, _: Context, ready: Ready) {
    if let Some(shard) = ready.shard {
      println!(
        "{} connected on shard {}/{}",
        ready.user.name,
        shard[0] + 1,
        shard[1],
      );
    }
  }
  fn message(&self, ctx: Context, msg: Message) {
    println!(
      "({id}) {name}#{discrim}: {content}",
      id = msg.author.id,
      name = msg.author.name,
      discrim = msg.author.discriminator,
      content = msg.content
    );

    // Ignore messages from the bot
    if msg.is_own(&ctx) {
      return;
    }

    let tim = UserId(83281345949728768).to_user(&ctx).unwrap();
    if msg.mentions.contains(&tim) {
      let _ = msg.channel_id.say(
        &ctx.http,
        "Imagine pinging Tim... <:haHaa:340276843523473409>",
      );

      return;
    }

    match automod(msg.content.clone()) {
      Ok(bool) => {
        if !bool {
          let _ = msg.delete(&ctx);
          match msg.reply(
            &ctx,
            "**Your message blocked by Automod. Please watch what you say.**",
          ) {
            Ok(msg) => {
              thread::spawn(move || {
                thread::sleep(Duration::from_millis(15000));
                let _ = msg.delete(&ctx);
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
