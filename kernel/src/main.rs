#![no_std]
#![no_main]
#![warn(clippy::pedantic)]
#![feature(decl_macro, lang_items, abi_x86_interrupt, custom_test_frameworks)]
#![test_runner(crate::tests::test_runner)]
#![reexport_test_harness_main = "test_main"]

use bootloader_api::{entry_point, info::Optional, BootInfo};
use core::panic::PanicInfo;

mod fb;
mod interrupts;
mod late_init;
mod logger;
#[cfg(test)]
mod tests;
mod util;

entry_point!(main);

fn main(boot_info: &'static mut BootInfo) -> ! {
    let framebuffer = core::mem::replace(&mut boot_info.framebuffer, Optional::None);
    let framebuffer = framebuffer.into_option();

    interrupts::init_idt();
    logger::init(framebuffer);

    #[cfg(test)]
    {
        test_main();
        tests::exit_qemu(tests::QemuExitCode::Success);
    }

    util::halt();
}

#[panic_handler]
fn panic_handler(info: &PanicInfo) -> ! {
    log::error!("{info}");

    #[cfg(test)]
    tests::exit_qemu(tests::QemuExitCode::Failed);

    util::halt();
}

#[lang = "eh_personality"]
fn eh_personality() {}
