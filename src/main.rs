// We deny unsafe code because we want to be as secure as possible.
// If we really need it we can allow it on a case by case basis.
#![deny(unsafe_code)]
#![forbid(clippy::undocumented_unsafe_blocks)]

use std::env;

use rocket::State;
use sqlx::{
    pool::{PoolConnection, PoolOptions},
    Pool,
};
#[macro_use]
extern crate rocket;

pub mod authserver;
pub mod model;
pub mod mojang;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

type Database = sqlx::sqlite::Sqlite;
type DbConn = PoolConnection<Database>;
type Db = State<Pool<Database>>;

#[rocket::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv()?;
    let database_url = env::var("DATABASE_URL")?;
    let pool = <PoolOptions<Database>>::new()
        .connect(&database_url)
        .await?;
    sqlx::migrate!("./migrations").run(&pool).await?;

    let _rocket = rocket::build()
        .manage(pool)
        .mount("/", routes![index])
        .mount(
            "/authserver",
            routes![authserver::authenticate::authenticate],
        )
        .launch()
        .await?;

    Ok(())
}
