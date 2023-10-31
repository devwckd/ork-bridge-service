use ork_bridge_service::domains::namespace::{Namespace, NamespaceError, NamespaceResult};
use uuid::Uuid;

#[derive(Clone)]
pub struct NamespaceRepository {
    pg_pool: sqlx::PgPool,
}

impl NamespaceRepository {
    pub fn new(pg_pool: sqlx::PgPool) -> Self {
        Self { pg_pool }
    }

    pub async fn find_by_id(&self, id: &Uuid) -> NamespaceResult<Namespace> {
        sqlx::query_as("SELECT * FROM namespaces WHERE id = $1;")
            .bind(&id)
            .fetch_optional(&self.pg_pool)
            .await?
            .ok_or(NamespaceError::NotFound)
    }

    pub async fn insert(&self, namespace: &Namespace) -> NamespaceResult<()> {
        sqlx::query("INSERT INTO namespaces(id, slug) VALUES ($1, $2)")
            .bind(&namespace.id)
            .bind(&namespace.slug)
            .execute(&self.pg_pool)
            .await?;

        Ok(())
    }
}
