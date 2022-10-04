use std::process::Command;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::new("qemu-system-x86_64");
    cmd.arg("-accel").arg(if cfg!(target_os = "windows") {
        "tcg"
    } else {
        "kvm"
    });
    cmd.arg("-serial").arg("stdio");
    if std::env::args().any(|arg| arg == "--bios") {
        cmd.arg("-drive").arg(format!(
            "format=raw,file={}",
            concat!(env!("OUT_DIR"), "/bios.img")
        ));
    } else {
        cmd.arg("-bios").arg(ovmf_prebuilt::ovmf_pure_efi());
        cmd.arg("-drive").arg(format!(
            "format=raw,file={}",
            concat!(env!("OUT_DIR"), "/uefi.img")
        ));
    }
    let mut child = cmd.spawn()?;
    child.wait()?;

    Ok(())
}
