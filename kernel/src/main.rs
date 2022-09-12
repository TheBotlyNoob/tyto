#![no_std]
#![no_main]
#![feature(lang_items)]

use core::panic::PanicInfo;

use bootloader_api::{
    config::Mapping,
    entry_point,
    info::{MemoryRegionKind, Optional},
    BootInfo, BootloaderConfig,
};

use logger::Color;

mod graphical;
mod logger;

const CONFIG: BootloaderConfig = {
    let mut config = BootloaderConfig::new_default();

    config.mappings.physical_memory = Some(Mapping::Dynamic);

    config
};

entry_point!(main, config = &CONFIG);

fn main(boot_info: &'static mut BootInfo) -> ! {
    let framebuffer = core::mem::replace(&mut boot_info.framebuffer, Optional::None);
    let framebuffer = framebuffer.into_option();

    logger::init(framebuffer);

    let prelease_str = if boot_info.api_version.pre_release() {
        "(prerelease)"
    } else {
        ""
    };
    log::info!(
        "Bootloader version: {}.{}.{} {}",
        boot_info.api_version.version_major(),
        boot_info.api_version.version_minor(),
        boot_info.api_version.version_patch(),
        prelease_str
    );

    let physical_memory_offset = boot_info
        .physical_memory_offset
        .into_option()
        .expect("the bootloader should map all physical memory for us");
    log::info!("Physical memory offset: {physical_memory_offset:#018x}");

    log::info!("Memory regions: {}", boot_info.memory_regions.len(),);

    // Merge contiguous memory regions of the same kind and log them.
    boot_info
        .memory_regions
        .sort_unstable_by_key(|region| region.start);
    let mut iter = boot_info.memory_regions.iter().copied();
    if let Some(mut prev) = iter.next() {
        for next in iter {
            if prev.end != next.start || prev.kind != next.kind {
                log::info!("{:#018x} - {:#018x}: {:?}", prev.start, prev.end, prev.kind,);

                prev = next;
            } else {
                prev.end = next.end;
            }
        }

        log::info!("{:#018x} - {:#018x}: {:?}", prev.start, prev.end, prev.kind,);
    }

    log::info!("Writing to usable memory regions");

    for region in boot_info
        .memory_regions
        .iter()
        .filter(|region| region.kind == MemoryRegionKind::Usable)
    {
        let addr = physical_memory_offset + region.start;
        let size = region.end - region.start;
        unsafe {
            core::ptr::write_bytes(addr as *mut u8, 0xff, size as usize);
        }
    }

    log::info!("Done!");

    loop {}
}

#[panic_handler]
fn panic_handler(info: &PanicInfo) -> ! {
    log::error!("{info}");

    loop {}
}

#[lang = "eh_personality"]
fn eh_personality() {}
