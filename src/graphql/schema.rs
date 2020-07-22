use async_graphql::Context;
use bb8_postgres::PostgresConnectionManager;
use postgres::NoTls;

pub type PostgresPool = bb8::Pool<PostgresConnectionManager<NoTls>>;

pub struct Guild {
  pub id: i64,
  pub name: String,
}

#[async_graphql::Object]
impl Guild {
  async fn id(&self) -> &i64 {
    &self.id
  }

  async fn created_at(&self) -> i64 {
    (&self.id >> 22) + 1420070400000
  }

  async fn name(&self) -> &str {
    &self.name
  }
}

pub struct QueryRoot;

#[async_graphql::Object]
impl QueryRoot {
  async fn guilds(&self, ctx: &Context<'_>) -> Vec<Guild> {
    let pool = ctx.data_unchecked::<PostgresPool>().get().await.unwrap();

    let guilds = pool
      .query("SELECT id, name FROM guilds", &[])
      .await
      .unwrap();
    let mut resp: Vec<Guild> = Vec::new();
    for guild in guilds {
      resp.push(Guild {
        id: guild.get(0),
        name: guild.get(1),
      });
    }

    resp
  }
}

pub struct MutationRoot;

#[async_graphql::Object]
impl MutationRoot {}

pub struct SubscriptionRoot;

#[async_graphql::Subscription]
impl SubscriptionRoot {}
