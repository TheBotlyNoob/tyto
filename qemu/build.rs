use std::{env::var, path::Path};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let out_dir = var("OUT_DIR")?;
    let out_dir = Path::new(&out_dir);
    std::fs::write(out_dir.join("OVMF.fd"), {
        let mut ovmf = Vec::new();
        ureq::get(
            "https://github.com/rust-osdev/ovmf-prebuilt/releases/latest/download/OVMF-pure-efi.fd",
        )
        .call()?
        .into_reader()
        .read_to_end(&mut ovmf)?;
        ovmf
    })?;

    Ok(())
}
