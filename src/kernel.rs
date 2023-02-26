use crate::api::BootSource;
use crate::instance::FcVmm;

impl FcVmm {
    pub async fn set_boot_source(&mut self, boot_source: BootSource) {
        self.client
            .put_guest_boot_source(boot_source)
            .await
            .unwrap();
    }
}
