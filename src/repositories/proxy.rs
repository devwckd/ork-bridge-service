use crate::domains::proxy::{Proxy, ProxyResult};
use sqlx::query;
use uuid::Uuid;

#[derive(Clone)]
pub struct ProxyRepository {
    pg_pool: sqlx::PgPool,
}

impl ProxyRepository {
    pub fn new(pg_pool: sqlx::PgPool) -> Self {
        Self { pg_pool }
    }

    pub async fn insert(&self, namespace_id: &Uuid, proxy: &Proxy) -> ProxyResult<()> {
        query("INSERT INTO proxies(id, slug, namespace_id) VALUES ($1, $2, $3);")
            .bind(&proxy.id)
            .bind(&proxy.slug)
            .bind(&namespace_id)
            .execute(&self.pg_pool)
            .await?;

        Ok(())
    }
}
