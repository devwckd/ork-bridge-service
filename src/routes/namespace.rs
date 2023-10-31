use crate::domains::namespace::{CreateNamespaceData, Namespace, NamespaceResult};
use crate::managers::namespace::NamespaceManager;
use axum::extract::State;
use axum::routing::post;
use axum::Json;
use uuid::Uuid;
use validator::Validate;

pub fn router(namespace_manager: NamespaceManager) -> axum::Router {
    let state = NamespaceState { namespace_manager };

    axum::Router::new()
        .route("/", post(create))
        .with_state(state)
}

async fn create(
    State(NamespaceState { namespace_manager }): State<NamespaceState>,
    Json(data): Json<CreateNamespaceData>,
) -> NamespaceResult<Json<Namespace>> {
    data.validate()?;

    let namespace = Namespace {
        id: Uuid::new_v4(),
        slug: data.slug.clone(),
    };

    namespace_manager.create(&namespace).await?;

    Ok(Json(namespace))
}

#[derive(Clone)]
struct NamespaceState {
    namespace_manager: NamespaceManager,
}
