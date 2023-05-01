use std::path::PathBuf;

use clap::Parser;
use fclib::api::{BootSource, Drive};
use fclib::FcVmm;

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

    let mut vmm = match FcVmm::builder(&args.fc_path, &api_socket)
        .with_log_level(fclib::LogLevel::Debug)
        .start_vmm()
    {
        Ok(vmm) => vmm,
        Err(err) => {
            println!("Could not start vmm: {err}");
            return;
        }
    };

    println!("Firecracker PID: {}", vmm.pid());

    let boot_source = BootSource::new(args.fc_path);
    vmm.set_boot_source(boot_source).await;

    let drive = Drive::new("1".to_owned(), true, true, args.rootfs_path);
    vmm.add_drive("1", drive).await;
}
