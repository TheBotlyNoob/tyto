use bootloader::{BiosBoot, UefiBoot};
use std::path::Path;

fn main() {
    let out_dir = var("OUT_DIR");
    let out_dir = Path::new(&out_dir);

    let kernel_file = var("CARGO_BIN_FILE_KERNEL");
    let kernel_file = Path::new(&kernel_file);

    UefiBoot::new(kernel_file)
        .create_disk_image(&out_dir.join("uefi.img"))
        .unwrap();
    BiosBoot::new(kernel_file)
        .create_disk_image(&out_dir.join("bios.img"))
        .unwrap();
}

fn var(s: impl AsRef<std::ffi::OsStr>) -> String {
    std::env::var(s).unwrap()
}
