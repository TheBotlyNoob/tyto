#![no_std]
#![no_main]
#![warn(clippy::pedantic)]
#![feature(decl_macro, lang_items)]

use core::panic::PanicInfo;

use bootloader_api::{entry_point, info::Optional, BootInfo};

use crate::logger::print;

mod fb;
mod late_init;
mod logger;

entry_point!(main);

fn main(boot_info: &'static mut BootInfo) -> ! {
    let framebuffer = core::mem::replace(&mut boot_info.framebuffer, Optional::None);
    let framebuffer = framebuffer.into_option();

    logger::init(framebuffer);

    // fb::FRAMEBUFFER.get().clear(Rgb888::GREEN).unwrap();

    log::info!("Done!");
    log::info!("another");
    for i in 0..=1000 {
        print!("-{i}");
    }

    loop {}
}

#[panic_handler]
fn panic_handler(info: &PanicInfo) -> ! {
    log::error!("{info}");

    loop {}
}

#[lang = "eh_personality"]
fn eh_personality() {}
