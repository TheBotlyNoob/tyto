use std::{
    path::Path,
    process::{Command, Stdio},
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Building the kernel...");
    let root = env!("CARGO_MANIFEST_DIR");
    let root = Path::new(&root).parent().unwrap();
    Command::new(env!("CARGO"))
        .args(&[
            "build",
            #[cfg(not(debug_assertions))]
            "--release",
            "--package=kernel",
            "--target=x86_64-unknown-uefi",
            "-Zbuild-std=core,compiler_builtins,alloc",
            "-Zbuild-std-features=compiler-builtins-mem",
        ])
        .current_dir(root)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()?
        .wait()?;

    let target_dir = root.join("target");

    let efi_dir = target_dir.join("efi");
    let boot_dir = efi_dir.join("EFI").join("Boot");
    std::fs::create_dir_all(&boot_dir)?;

    std::fs::copy(
        target_dir
            .join("x86_64-unknown-uefi")
            .join(if cfg!(debug_assertions) {
                "debug"
            } else {
                "release"
            })
            .join("kernel.efi"),
        boot_dir.join("BOOTX64.EFI"),
    )?;

    if let Some(arg) = std::env::args().nth(1) && arg == "--no-run" {
        println!("Not launching qemu...");
        return Ok(());
    }

    println!("Launching qemu...");

    let qemu_args = &[
        "-nodefaults",
        "-machine",
        "q35",
        "-smp",
        "4",
        "-m",
        "256M",
        "-device",
        "isa-debug-exit,iobase=0xf4,iosize=0x04",
        "-bios",
        concat!(env!("OUT_DIR"), "/OVMF.fd"),
        "-drive",
        "file=fat:rw:target/efi,format=raw",
        "-vga",
        "std",
        "-serial",
        "stdio",
    ];

    println!("qemu-system-x86_64 {}", qemu_args.join(" "));

    Command::new("qemu-system-x86_64")
        .args(qemu_args)
        .current_dir(root)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()?
        .wait()?;

    Ok(())
}
