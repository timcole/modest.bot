use serenity::{
  client::Context,
  framework::standard::{macros::help, Args, CommandGroup, CommandResult, HelpOptions},
  model::prelude::*,
};
use std::collections::HashSet;

#[help]
async fn help(
  ctx: &Context,
  msg: &Message,
  _: Args,
  _: &'static HelpOptions,
  groups: &[&'static CommandGroup],
  owners: HashSet<UserId>,
) -> CommandResult {
  let mut fields = HashSet::new();

  for group in groups {
    if group.options.owners_only && !owners.contains(&msg.author.id) {
      continue;
    }

    let mut commands = String::from("```\n");
    for y in group.options.commands {
      commands.push_str(&format!("{} ", &y.options.names.first().unwrap()[..]));
    }
    commands.push_str("\n```");

    fields.insert((group.name, commands, false));
  }

  msg
    .channel_id
    .send_message(&ctx.http, |m| {
      m.embed(|e| {
        e.author(|a| {
          a.name("Created by Tim#6969")
            .icon_url("https://github.com/timcole.png")
            .url("https://twitter.com/modesttim")
        })
        .fields(fields)
        .footer(|f| f.text(format!("Commit Hash: {}", &env!("GIT_HASH")[0..7])))
      })
    })
    .await?;

  Ok(())
}
