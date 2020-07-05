use serenity::framework::standard::macros::group;

// pub mod general;

// #[group]
// #[commands()]
// struct General;

pub mod admin;
use admin::{kill::*, ping::*};

#[group]
#[commands(ping, kill)]
#[owners_only]
struct Admin;

pub mod help;
