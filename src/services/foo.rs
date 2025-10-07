use tracing::info;

use crate::core::Result;
use crate::core::state::AppState;
use crate::ok;
use crate::types::foo::FooRequest;
use crate::types::foo::FooResponse;

/// foo service implement method
pub struct FooService;

impl FooService {
    #[allow(unused)]
    #[tracing::instrument(skip(state))]
    pub async fn foo(state: AppState, req: FooRequest) -> Result<FooResponse> {
        info!("req {}", req.key_word);
        ok!(FooResponse::default())
    }
}
