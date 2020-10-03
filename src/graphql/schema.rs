use async_graphql::{Context, FieldError, FieldResult};
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
  async fn guild(&self, ctx: &Context<'_>, id: i64) -> FieldResult<Guild> {
    let pool = match ctx.data_unchecked::<PostgresPool>().get().await {
      Ok(pool) => pool,
      Err(_) => return Err(FieldError("Fatal db pool error".to_string(), None)),
    };

    let guild = match pool
      .query("SELECT id, name FROM guilds WHERE id = $1", &[&id])
      .await
    {
      Ok(guild) => guild,
      Err(e) => {
        println!("{}", e);
        return Err(FieldError("Failed to fetch guild".to_string(), None));
      }
    };

    if guild.len() == 0 {
      return Err(FieldError("Guild not found".to_string(), None));
    }

    let guild = &guild[0];

    Ok(Guild {
      id: guild.get(0),
      name: guild.get(1),
    })
  }
}

pub struct MutationRoot;

#[async_graphql::Object]
impl MutationRoot {}

pub struct SubscriptionRoot;

#[async_graphql::Subscription]
impl SubscriptionRoot {}
