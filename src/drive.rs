use crate::api::Drive;
use crate::instance::FcVmm;

impl FcVmm {
    pub async fn add_drive(&mut self, drive_id: &str, drive: Drive) {
        self.client
            .put_guest_drive_by_id(drive_id, drive)
            .await
            .unwrap();
    }
}
