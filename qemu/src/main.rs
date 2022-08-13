use std::{
    path::Path,
    process::{Command, Stdio},
};

use cargo_metadata::Message;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Building the kernel...");
    let root = env!("CARGO_MANIFEST_DIR");
    let root = Path::new(&root).parent().unwrap();
    let args = std::env::args().collect::<Vec<_>>();

    let target_dir = root.join("target");
    let bin = if args.contains(&String::from("--test")) {
        let output = {
            let mut cmd = Command::new(env!("CARGO"));
            cmd.args(&[
                "test",
                #[cfg(not(debug_assertions))]
                "--release",
                "--no-run",
                "--message-format=json",
                "--package=kernel",
                "--target=x86_64-unknown-uefi",
                "-Zbuild-std=core,compiler_builtins,alloc",
                "-Zbuild-std-features=compiler-builtins-mem",
            ])
            .current_dir(root)
            .stdout(Stdio::piped())
            .stderr(Stdio::null());

            println!("{cmd:#?}");
            cmd.output()?
        };
        let mut path = None;
        for message in Message::parse_stream(&*output.stdout).map(Result::unwrap) {
            if let Message::CompilerArtifact(artifact) = message {
                if artifact.target.name == "kernel" && let Some(exe) = artifact.executable {
                    path.replace(exe.into_std_path_buf());
                    break;
                }
            }
        }
        match path {
            Some(path) => path,
            None => {
                // run the command again to get the error message
                let _ = Command::new(env!("CARGO"))
                    .args(&[
                        "test",
                        #[cfg(not(debug_assertions))]
                        "--release",
                        "--no-run",
                        "--package=kernel",
                        "--target=x86_64-unknown-uefi",
                        "-Zbuild-std=core,compiler_builtins,alloc",
                        "-Zbuild-std-features=compiler-builtins-mem",
                    ])
                    .current_dir(root)
                    .stdout(Stdio::inherit())
                    .stderr(Stdio::inherit())
                    .spawn()?
                    .wait();
                std::process::exit(1);
            }
        }
    } else {
        let output = {
            let mut cmd = Command::new(env!("CARGO"));
            cmd.args(&[
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
            .stderr(Stdio::inherit());

            println!("{cmd:#?}");
            cmd.status()?
        };

        if !output.success() {
            std::process::exit(1);
        }

        target_dir
            .join("x86_64-unknown-uefi")
            .join(if cfg!(debug_assertions) {
                "debug"
            } else {
                "release"
            })
            .join("kernel.efi")
    };

    let efi_dir = target_dir.join("efi");
    let boot_dir = efi_dir.join("EFI").join("Boot");
    std::fs::create_dir_all(&boot_dir)?;

    std::fs::copy(bin, boot_dir.join("BOOTX64.EFI"))?;

    if args.contains(&String::from("--no-run")) {
        println!("Not launching qemu...");
        return Ok(());
    }

    println!("Launching qemu...");

    let mut qemu_args = vec![
        "-nodefaults",
        "-machine",
        "q35",
        "-smp",
        "4",
        "-m",
        "256M",
        "-bios",
        concat!(env!("OUT_DIR"), "/OVMF.fd"),
        "-drive",
        "file=fat:rw:target/efi,format=raw",
        "-vga",
        "std",
        "-serial",
        "stdio",
    ];

    if args.contains(&String::from("--test")) {
        qemu_args.extend([
            "-device",
            "isa-debug-exit,iobase=0xf4,iosize=0x04",
            "-display",
            "none",
        ]);
    }

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
