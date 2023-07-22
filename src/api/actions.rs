use super::client::ApiClient;
use super::{InstanceAction, InstanceActionInfo, Result};

impl ApiClient {
    async fn do_action(&self, action: InstanceAction) -> Result<()> {
        let action = InstanceActionInfo::new(action);
        self.put("/actions", action).await
    }

    pub async fn start_microvm(&self) -> Result<()> {
        self.do_action(InstanceAction::InstanceStart).await
    }

    pub async fn stop_microvm(&self) -> Result<()> {
        self.do_action(InstanceAction::SendCtrlAltDel).await
    }

    pub async fn flush_metrics(&self) -> Result<()> {
        self.do_action(InstanceAction::FlushMetrics).await
    }
}
