use crate::auth::Claims;
use crate::core::Result;
use crate::core::state::AppState;
use crate::core::valid::ValidQuery;
use crate::types::foo::FooRequest;
use crate::types::foo::FooResponse;
use axum::debug_handler;
use axum::extract::State;
use tracing::debug;

#[debug_handler]
pub async fn foo(
    _claims: Claims,
    State(state): State<AppState>,
    ValidQuery(req): ValidQuery<FooRequest>,
) -> Result<FooResponse> {
    debug!("foo request {req:?}");
    state.foo_service.foo(req).await
}
