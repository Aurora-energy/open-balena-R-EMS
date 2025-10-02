//! Simplified job orchestration logic.
//!
//! The intent of this module is to show how a dedicated orchestrator could sit
//! between the API layer and lower-level execution engines. By abstracting the
//! queueing mechanism we can swap AMQP, Redis Streams, or other transports
//! without modifying business logic.

use async_trait::async_trait;
use thiserror::Error;
use tracing::instrument;

/// Represents any backend capable of accepting deployment requests. During
/// early development we can mock this trait in tests while exploring UX.
#[async_trait]
pub trait DeploymentQueue: Send + Sync {
    /// Queues a deployment operation for the given device and image reference.
    async fn enqueue(&self, device_id: &str, image: &str) -> Result<(), OrchestratorError>;
}

/// Basic implementation that only logs intent. Replace with a concrete queue
/// client when integrating with infrastructure.
#[derive(Default)]
pub struct JobOrchestrator;

#[async_trait]
impl DeploymentQueue for JobOrchestrator {
    #[instrument(skip(self))]
    async fn enqueue(&self, device_id: &str, image: &str) -> Result<(), OrchestratorError> {
        tracing::info!(%device_id, %image, "queueing deployment job");
        Ok(())
    }
}

impl JobOrchestrator {
    /// Helper wrapper that aligns with the CLI to improve readability.
    pub async fn queue_deployment(
        &self,
        device_id: &str,
        image: &str,
    ) -> Result<(), OrchestratorError> {
        self.enqueue(device_id, image).await
    }
}

/// Domain-specific errors make it easier to surface actionable messages to the
/// CLI or API consumers.
#[derive(Debug, Error)]
pub enum OrchestratorError {
    /// Represents failures returned from the underlying queue transport.
    #[error("failed to enqueue job: {0}")]
    Transport(String),
}
