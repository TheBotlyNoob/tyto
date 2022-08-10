#![no_main]
#![no_std]
#![feature(abi_efiapi, abi_x86_interrupt)]

#[cfg(not(all(target_arch = "x86_64", target_vendor = "unknown", target_os = "uefi")))]
compile_error!(concat!(
    "Targets other than `x86_64-unknown-uefi` are not supported",
    "\n",
    "Are you using `cargo build`? Try `cargo kbuild` instead."
));

use embedded_graphics::{
    pixelcolor::Rgb888,
    prelude::{DrawTarget, RgbColor},
};
use uefi::prelude::*;

pub mod framebuffer;
pub mod keyboard;
pub mod late_init;
pub mod log;

#[entry]
fn main(_handle: Handle, mut system_table: SystemTable<Boot>) -> Status {
    let mut fb = framebuffer::FrameBuffer::new(&mut system_table);
    fb.clear(Rgb888::BLACK).unwrap();
    log::Logger::init(fb.clone());

    println!("Hello, world!");
    println!("af\nter");

    loop {
        x86_64::instructions::hlt();
    }
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    println!("{info}");
    loop {}
}
