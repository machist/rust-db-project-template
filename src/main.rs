use anyhow::Context;
use clap::Parser;

use rust_db_project_template::config::Config;
use sqlx::postgres::PgPoolOptions;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // environment
    dotenv::dotenv().ok();

    // logger
    env_logger::init();

    // config
    let config = Config::parse();

    // db
    let db = PgPoolOptions::new()
        .max_connections(50)
        .connect(&config.database_url)
        .await
        .context("could not connect to database_url")?;

    // migrate
    sqlx::migrate!().run(&db).await?;

    // ok
    Ok(())
}
