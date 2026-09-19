use axum::debug_handler;
use axum::extract::Query;
use axum::extract::State;
use axum_valid::Valid;
use tracing::debug;

use crate::auth::Claims;
use crate::core::Result;
use crate::core::state::AppState;
use crate::types::foo::FooRequest;
use crate::types::foo::FooResponse;

#[debug_handler]
pub async fn foo(
    _claims: Claims,
    State(state): State<AppState>,
    Valid(Query(req)): Valid<Query<FooRequest>>,
) -> Result<FooResponse> {
    debug!("foo request {req:?}");
    state.foo_service.foo(req).await
}
