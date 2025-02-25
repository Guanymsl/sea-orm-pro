use loco_rs::cli;
use loco_starter::app::App;
use migration::Migrator;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    dotenvy::dotenv().ok();

    let mysql_host = var("DATABASE_URL").expect("DATABASE_URL is not set");
    println!("MySQL Host: {}", mysql_host);
    Database::connect(mysql_host.as_str()).await?;

    cli::main::<App, Migrator>().await?;
    Ok(())
}
