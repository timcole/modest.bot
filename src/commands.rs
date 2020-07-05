use serenity::framework::standard::macros::group;

pub mod commands;
use commands::ping::*;

#[group]
#[commands(ping)]
struct General;
