use loco_rs::{environment::Environment, prelude::*};
use sea_orm_pro::{ConfigParser, JsonCfg};
use seaography::lazy_static;

const CONFIG_ROOT: &str = "pro_admin";

lazy_static::lazy_static! {
    static ref CONFIG: JsonCfg = ConfigParser::new().load_config(CONFIG_ROOT).unwrap();
}

pub async fn config(State(ctx): State<AppContext>) -> Result<Response> {
    if ctx.environment == Environment::Production {
        // Release: load config from the disk once and then return the cached config afterwards
        format::json(&*CONFIG)
    } else {
        // Debug: load config from disk on every request
        let config = ConfigParser::new()
            .load_config(CONFIG_ROOT)
            .map_err(Into::<Box<dyn std::error::Error + Send + Sync>>::into)?;
        format::json(config)
    }
}

pub fn routes() -> Routes {
    Routes::new()
        // Admin route prefix
        .prefix("admin")
        // Fetch web config
        .add("/config", get(config))
}