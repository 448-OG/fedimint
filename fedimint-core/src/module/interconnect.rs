use async_trait::async_trait;

use super::ApiError;
use crate::core::ModuleInstanceId;
use crate::module::ApiRequestErased;

/// Provides an interface to call APIs of other modules
#[async_trait]
pub trait ModuleInterconect: Sync + Send {
    /// Simulates a call to an API endpoint of another module.
    /// This has lower latency.
    async fn call(
        &self,
        module_id: ModuleInstanceId,
        path: String,
        data: ApiRequestErased,
    ) -> Result<serde_json::Value, ApiError>;
}
