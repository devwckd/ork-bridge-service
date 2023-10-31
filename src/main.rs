mod managers;
mod repositories;
mod routes;

use crate::managers::namespace::NamespaceManager;
use crate::managers::proxy::ProxyManager;
use crate::repositories::namespace::NamespaceRepository;
use crate::repositories::proxy::ProxyRepository;
use std::net::SocketAddr;
use tower_http::trace::{DefaultMakeSpan, DefaultOnResponse, TraceLayer};
use tracing::info;

#[tokio::main]
async fn main() {
    init_logging();

    let address: SocketAddr = "127.0.0.1:8081".parse().unwrap();

    let pg_pool = create_pg_pool().await;

    let namespace_repository = NamespaceRepository::new(pg_pool.clone());
    let proxy_repository = ProxyRepository::new(pg_pool.clone());

    let namespace_manager = NamespaceManager::new(namespace_repository.clone());
    let proxy_manager = ProxyManager::new(proxy_repository.clone());

    let router = axum::Router::new()
        .nest(
            "/namespaces",
            routes::namespace::router(namespace_manager.clone()),
        )
        .nest(
            "/proxies",
            routes::proxy::router(namespace_manager.clone(), proxy_manager.clone()),
        )
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::new().level(tracing::Level::INFO))
                .on_response(DefaultOnResponse::new().level(tracing::Level::INFO)),
        );

    info!("binding on {}", &address);

    axum::Server::bind(&address)
        .serve(router.into_make_service())
        .await
        .unwrap();
}

fn init_logging() {
    tracing_subscriber::fmt::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();
}

async fn create_pg_pool() -> sqlx::PgPool {
    let pg_pool = sqlx::PgPool::connect("postgres://postgres:secret@localhost:5432/bridge_service")
        .await
        .unwrap();

    sqlx::migrate!().run(&pg_pool).await.unwrap();

    pg_pool
}
