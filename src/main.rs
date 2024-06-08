use std::sync::Arc;

use axum::{
    extract::State,
    http::StatusCode,
    Json,
    response::IntoResponse,
    Router, routing::get,
};
use serde::Deserialize;
use tokio::sync::RwLock;

#[tokio::main]
async fn main() {
    let global_price = Arc::new(RwLock::new(None));
    let app = app(global_price);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}

fn app(state: GlobalPrice) -> Router {
    Router::new()
        .route("/price", get(get_price).patch(set_price).delete(set_null_price))
        .with_state(state)
}

async fn get_price(
    State(global_price): State<GlobalPrice>,
) -> Result<impl IntoResponse, StatusCode> {
    let global_price = global_price.read().await;
    if let Some(price) = *global_price {
        Ok(price.to_string())
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}

#[derive(Debug, Deserialize)]
struct PriceDto {
    price: u64,
}

async fn set_price(
    State(global_price): State<GlobalPrice>,
    Json(input): Json<PriceDto>,
) -> Result<impl IntoResponse, StatusCode> {
    let price = input.price;
    let mut global_price = global_price.write().await;
    *global_price = Some(price);

    Ok(StatusCode::OK)
}

async fn set_null_price(
    State(global_price): State<GlobalPrice>,
) -> Result<impl IntoResponse, StatusCode> {
    let mut global_price = global_price.write().await;
    *global_price = None;

    Ok(StatusCode::OK)
}

type GlobalPrice = Arc<RwLock<Option<u64>>>;
