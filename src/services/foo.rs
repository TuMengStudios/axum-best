use tracing::info;

use crate::core::Result;
use crate::ok;
use crate::types::foo::FooRequest;
use crate::types::foo::FooResponse;

/// foo service implement method
#[derive(Clone, Default)]
pub struct FooService;

impl FooService {
    #[allow(unused)]
    #[tracing::instrument(skip(self))]
    pub async fn foo(&self, req: FooRequest) -> Result<FooResponse> {
        info!("req {}", req.key_word);
        ok!(FooResponse::default())
    }
}
