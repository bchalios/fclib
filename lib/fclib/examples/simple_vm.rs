use std::path::PathBuf;
use std::time::Duration;

use clap::Parser;
use fclib::client::drive::Drive;
use fclib::client::kernel::BootSource;
use fclib::vmm::{LogLevel, Vmm};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct CliArgs {
    /// Path to Firecracker binary
    #[arg(short, long, default_value_t = String::from("./firecracker"))]
    fc_path: String,

    /// Path to Linux kernel
    #[arg(short, long, default_value_t = String::from("./vmlinux.bin"))]
    vmlinux_path: String,

    /// Path to rootfs
    #[arg(short, long, default_value_t = String::from("./rootfs.ext4"))]
    rootfs_path: String,
}

#[tokio::main]
async fn main() {
    let args = CliArgs::parse();

    env_logger::init();

    let api_socket = PathBuf::from("/tmp/firecracker.socket");
    if api_socket.exists() {
        std::fs::remove_file(&api_socket).unwrap();
    }

    let mut vmm = match Vmm::builder(&args.fc_path, &api_socket)
        .with_log_level(LogLevel::Debug)
        .start_vmm()
    {
        Ok(vmm) => vmm,
        Err(err) => {
            println!("Could not start vmm: {err}");
            return;
        }
    };
    let mut client = vmm.api_client();

    println!("Firecracker PID: {}", vmm.pid());

    let kernel = BootSource {
        kernel_image_path: args.vmlinux_path,
        boot_args: Some("console=ttyS0 reboot=k panic=1 pci=off".to_owned()),
        initrd_path: None,
    };
    client.set_boot_source(&kernel).await.unwrap();

    let drive = Drive::new("rootfs".to_string(), args.rootfs_path, true, false);
    client.add_drive("rootfs", &drive).await.unwrap();

    client.start_microvm().await.unwrap();

    let mut stdout = String::new();
    loop {
        vmm.serial_out(&mut stdout).unwrap();
        println!("{stdout}");
        std::thread::sleep(Duration::from_secs(1));
    }
}
