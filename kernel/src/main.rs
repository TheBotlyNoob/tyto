#![no_std]
#![no_main]
#![warn(clippy::pedantic, clippy::nursery)]
#![feature(
    decl_macro,
    lang_items,
    abi_x86_interrupt,
    custom_test_frameworks,
    stmt_expr_attributes
)]
#![test_runner(crate::tests::test_runner)]
#![reexport_test_harness_main = "test_main"]

use bootloader_api::{entry_point, info::Optional, BootInfo};
use core::panic::PanicInfo;
use embedded_graphics::{
    pixelcolor::Rgb888,
    prelude::{DrawTarget, RgbColor},
};

mod data;
mod fb;
mod interrupts;
mod logger;
#[cfg(test)]
mod tests;

entry_point!(main);

fn main(boot_info: &'static mut BootInfo) -> ! {
    let framebuffer = core::mem::replace(&mut boot_info.framebuffer, Optional::None).into_option();

    interrupts::init_idt();
    fb::init_framebuffer(framebuffer);
    logger::init();

    fb::FRAMEBUFFER.fb.lock().clear(Rgb888::WHITE).unwrap();

    // log::info!("Hello World{}", "!");

    cfg_if::cfg_if! {
        if #[cfg(test)] {
            test_main();
            tests::exit_qemu(tests::QemuExitCode::Success);
        } else {
            halt();
        }
    };
}

#[panic_handler]
fn panic_handler(info: &PanicInfo) -> ! {
    log::error!("{info}");

    #[cfg(test)]
    tests::exit_qemu(tests::QemuExitCode::Failed);

    halt();
}

pub fn halt() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}

#[lang = "eh_personality"]
const fn eh_personality() {
    // TODO: stack unwinding
}
