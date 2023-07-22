use super::client::ApiClient;
use super::{Balloon, BalloonStats, BalloonStatsUpdate, BalloonUpdate, Result};

impl ApiClient {
    pub async fn describe_balloon_config(&self) -> Result<Balloon> {
        self.get("/balloon").await
    }

    pub async fn put_balloon(&self, balloon: Balloon) -> Result<()> {
        self.put("/balloon", balloon).await
    }

    pub async fn patch_balloon(&self, balloon_update: BalloonUpdate) -> Result<()> {
        self.patch("/balloon", balloon_update).await
    }

    pub async fn patch_balloon_stats_interval(
        &self,
        balloon_stats_update: BalloonStatsUpdate,
    ) -> Result<()> {
        self.patch("/balloon/statistics", balloon_stats_update)
            .await
    }

    pub async fn describe_balloon_stats(&self) -> Result<BalloonStats> {
        self.get("/ballon/statistics").await
    }
}
