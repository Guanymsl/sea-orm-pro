use loco_rs::{cli, Result};
use migration::Migrator;
use loco_starter::app::App;

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();

    cli::main::<App, Migrator>().await
}
