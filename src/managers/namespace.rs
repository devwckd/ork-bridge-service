use crate::domains::namespace::{Namespace, NamespaceResult};
use crate::repositories::namespace::NamespaceRepository;
use uuid::Uuid;

#[derive(Clone)]
pub struct NamespaceManager {
    namespace_repository: NamespaceRepository,
}

impl NamespaceManager {
    pub fn new(namespace_repository: NamespaceRepository) -> Self {
        Self {
            namespace_repository,
        }
    }

    pub async fn find_by_id(&self, id: &Uuid) -> NamespaceResult<Namespace> {
        self.namespace_repository.find_by_id(id).await
    }

    pub async fn create(&self, namespace: &Namespace) -> NamespaceResult<()> {
        self.namespace_repository.insert(namespace).await
    }
}
