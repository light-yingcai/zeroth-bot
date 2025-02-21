mod actuator;
mod firmware;

pub use actuator::*;
pub use firmware::*;

use kos_core::kos_proto::actuator::actuator_service_server::ActuatorServiceServer;
use kos_core::services::{ActuatorServiceImpl, OperationsServiceImpl};
use kos_core::{Platform, ServiceEnum};
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::time::Duration;
use tonic::async_trait;

pub struct ZBotPlatform {}

impl ZBotPlatform {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for ZBotPlatform {
    fn default() -> Self {
        ZBotPlatform::new()
    }
}

#[async_trait]
impl Platform for ZBotPlatform {
    fn name(&self) -> &'static str {
        "ZBot"
    }

    fn serial(&self) -> String {
        std::fs::read_to_string("/tmp/mac1")
            .map(|mac| mac.trim().replace(":", ""))
            .unwrap_or_else(|_| "00000000".to_string())
    }

    fn initialize(&mut self, _operations_service: Arc<OperationsServiceImpl>) -> eyre::Result<()> {
        Ok(())
    }

    fn create_services<'a>(
        &'a self,
        _operations_service: Arc<OperationsServiceImpl>,
    ) -> Pin<Box<dyn Future<Output = eyre::Result<Vec<ServiceEnum>>> + Send + 'a>> {
        Box::pin(async move {
            let actuator = ZBotActuator::new().await?;
            Ok(vec![ServiceEnum::Actuator(ActuatorServiceServer::new(
                ActuatorServiceImpl::new(Arc::new(actuator)),
            ))])
        })
    }

    fn shutdown(&mut self) -> eyre::Result<()> {
        Ok(())
    }
}
