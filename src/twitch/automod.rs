use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Serialize, Deserialize)]
struct Data {
  msg_id: String,
  user_id: String,
  msg_text: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Body {
  data: Vec<Data>,
}

#[derive(Debug, Serialize, Deserialize)]
struct RespData {
  msg_id: String,
  is_permitted: bool,
}

#[derive(Debug, Serialize, Deserialize)]
struct Resp {
  data: Vec<RespData>,
}

pub async fn automod(msg: String) -> Result<bool, reqwest::Error> {
  let client_id: String = env::var("TWITCH_CLIENT").expect("Missing twitch client");
  let bearer: String = format!(
    "Bearer {}",
    dotenv::var("TWITCH_BEARER").expect("Missing twitch bearer")
  );

  let client = reqwest::Client::new();
  let automod_resp: Resp = client
    .post("https://api.twitch.tv/helix/moderation/enforcements/status?broadcaster_id=51684790")
    .json(&Body {
      data: vec![Data {
        msg_id: String::from("1"),
        user_id: String::from("1"),
        msg_text: msg,
      }],
    })
    .header("Client-ID", client_id)
    .header("Authorization", bearer)
    .send()
    .await?
    .json()
    .await?;

  Ok(automod_resp.data[0].is_permitted)
}
