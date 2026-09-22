use axum::Router;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::core::state::AppState;

/// Adds Swagger UI and the OpenAPI document when explicitly enabled.
pub fn apply(router: Router<AppState>, state: &AppState) -> Router<AppState> {
    if state.cfg.swagger.enabled {
        router.merge(
            SwaggerUi::new("/swagger-ui")
                .url("/api-docs/openapi.json", crate::docs::openapi::ApiDoc::openapi()),
        )
    } else {
        router
    }
}
