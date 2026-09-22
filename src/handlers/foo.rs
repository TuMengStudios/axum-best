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
#[utoipa::path(
    get,
    path = "/foo",
    tag = "Demo",
    summary = "Search Foo data",
    description = "Searches Foo data by keyword. This example endpoint requires an authenticated user.",
    security(("bearerAuth" = [])),
    params(FooRequest),
    responses(
        (status = 200, description = "Search successful", body = FooResponse),
        (status = 400, description = "Keyword validation failed"),
        (status = 401, description = "Missing or invalid authentication")
    )
)]
pub async fn foo(
    _claims: Claims,
    State(state): State<AppState>,
    ValidQuery(req): ValidQuery<FooRequest>,
) -> Result<FooResponse> {
    debug!("foo request {req:?}");
    state.foo_service.foo(req).await
}
