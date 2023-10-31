use crate::domains::namespace::CreateNamespaceData;
use crate::domains::proxy::{Proxy, ProxyResult};
use crate::managers::namespace::NamespaceManager;
use crate::managers::proxy::ProxyManager;
use axum::extract::{Path, State};
use axum::Json;
use uuid::Uuid;
use validator::Validate;

pub fn router(namespace_manager: NamespaceManager, proxy_manager: ProxyManager) -> axum::Router {
    let state = ProxyState {
        namespace_manager,
        proxy_manager,
    };

    axum::Router::new().with_state(state)
}

async fn create(
    State(ProxyState {
        namespace_manager,
        proxy_manager,
    }): State<ProxyState>,
    Path((namespace_id,)): Path<(Uuid,)>,
    Json(data): Json<CreateNamespaceData>,
) -> ProxyResult<Json<Proxy>> {
    data.validate()?;

    let namespace = namespace_manager.find_by_id(&namespace_id).await?;

    let proxy = Proxy {
        id: Uuid::new_v4(),
        slug: data.slug,
    };

    proxy_manager.create(&namespace.id, &proxy).await?;

    Ok(Json(proxy))
}

#[derive(Clone)]
struct ProxyState {
    namespace_manager: NamespaceManager,
    proxy_manager: ProxyManager,
}
