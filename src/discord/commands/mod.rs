use serenity::framework::standard::macros::group;

// pub mod general;

// #[group]
// #[commands()]
// struct General;

pub mod admin;
use admin::{kill::*, ping::*, sql::*};

#[group]
#[commands(ping, kill, sql)]
#[owners_only]
struct Admin;

pub mod help;
