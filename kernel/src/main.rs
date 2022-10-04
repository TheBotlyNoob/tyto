#![no_std]
#![no_main]
#![warn(clippy::pedantic)]
#![feature(decl_macro, lang_items, abi_x86_interrupt, custom_test_frameworks)]
#![test_runner(crate::tests::test_runner)]
#![reexport_test_harness_main = "test_main"]

use bootloader_api::{entry_point, info::Optional, BootInfo};
use core::panic::PanicInfo;

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

    log::info!("Hello World{}", "!");

    #[cfg(test)]
    {
        test_main();
        tests::exit_qemu(tests::QemuExitCode::Success);
    }
    #[cfg(not(test))]
    halt();
}

#[panic_handler]
fn panic_handler(info: &PanicInfo) -> ! {
    log::error!("{info}");

    #[cfg(test)]
    tests::exit_qemu(tests::QemuExitCode::Failed);

    halt();
}

#[lang = "eh_personality"]
fn eh_personality() {}

pub fn halt() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}
