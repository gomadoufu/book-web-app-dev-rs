use sqlx::{PgPool, postgres::PgConnectOptions};
use std::net::{Ipv4Addr, SocketAddr};

use anyhow::Result;
use axum::{Router, extract::State, http::StatusCode, routing::get};
use tokio::net::TcpListener;

pub async fn health_check() -> StatusCode {
    StatusCode::OK
}

async fn health_check_db(State(db): State<PgPool>) -> StatusCode {
    let connection_result = sqlx::query("SELECT 1").fetch_one(&db).await;
    match connection_result {
        Ok(_) => StatusCode::OK,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let database_cfg = DatabaseConfig {
        host: "localhost".to_string(),
        port: 5432,
        username: "app".to_string(),
        password: "passwd".to_string(),
        database: "app".to_string(),
    };

    let conn_pool = connect_database_with(database_cfg);

    let app = Router::new()
        .route("/health", get(health_check))
        .route("/health/db", get(health_check_db))
        .with_state(conn_pool);

    let addr = SocketAddr::new(Ipv4Addr::LOCALHOST.into(), 8080);

    let listener = TcpListener::bind(&addr).await?;
    Ok(axum::serve(listener, app).await?)
}

#[tokio::test]
async fn health_check_works() {
    let status_code = health_check().await;
    assert_eq!(status_code, StatusCode::OK);
}

#[sqlx::test]
async fn health_check_db_works(pool: sqlx::PgPool) {
    let status_code = health_check_db(State(pool)).await;
    assert_eq!(status_code, StatusCode::OK);
}
