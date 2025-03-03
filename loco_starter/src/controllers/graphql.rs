use axum::{body::Body, extract::Request};
use loco_rs::prelude::*;
use tower_service::Service;

use crate::graphql::query_root;

async fn graphql_handler(
    _auth: auth::JWT,
    State(ctx): State<AppContext>,
    req: Request<Body>,
) -> Result<Response> {
    // Maximum depth of the constructed query
    const DEPTH: Option<usize> = None;
    // Maximum complexity of the constructed query
    const COMPLEXITY: Option<usize> = None;
    // GraphQL schema
    let schema = query_root::schema(ctx.db.clone(), DEPTH, COMPLEXITY).unwrap();
    // GraphQL handler
    let mut graphql_handler = async_graphql_axum::GraphQL::new(schema);
    let res = graphql_handler.call(req).await.unwrap();

    Ok(res)
}

pub fn routes() -> Routes {
    Routes::new()
        // GraphQL route prefix
        .prefix("graphql")
        // Handling GraphQL request
        .add("/", post(graphql_handler))
}
