#![no_std]
#![no_main]
#![warn(clippy::pedantic)]
#![feature(decl_macro, lang_items, unsized_fn_params)]

use core::panic::PanicInfo;

use bootloader_api::{config::Mapping, entry_point, info::Optional, BootInfo, BootloaderConfig};

mod fb;
mod late_init;
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

    // fb::FRAMEBUFFER.get().clear(Rgb888::GREEN).unwrap();

    log::info!("Done!");
    log::info!("another");

    loop {}
}

#[panic_handler]
fn panic_handler(info: &PanicInfo) -> ! {
    log::error!("{info}");

    loop {}
}

#[lang = "eh_personality"]
fn eh_personality() {}
