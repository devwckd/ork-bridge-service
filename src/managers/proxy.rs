use crate::domains::proxy::{Proxy, ProxyResult};
use crate::repositories::proxy::ProxyRepository;
use uuid::Uuid;

#[derive(Clone)]
pub struct ProxyManager {
    proxy_repository: ProxyRepository,
}

impl ProxyManager {
    pub fn new(proxy_repository: ProxyRepository) -> Self {
        Self { proxy_repository }
    }

    pub async fn create(&self, namespace_id: &Uuid, proxy: &Proxy) -> ProxyResult<()> {
        self.proxy_repository.insert(namespace_id, proxy).await
    }
}
