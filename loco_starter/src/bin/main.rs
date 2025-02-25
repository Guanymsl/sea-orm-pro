use loco_rs::cli;
use loco_starter::app::App;
use migration::Migrator;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    dotenvy::dotenv().ok();

    cli::main::<App, Migrator>().await?;
    Ok(())
}
