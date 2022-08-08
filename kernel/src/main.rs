#![no_main]
#![no_std]
#![feature(abi_efiapi)]

#[cfg(not(all(target_arch = "x86_64", target_vendor = "unknown", target_os = "uefi")))]
compile_error!(concat!(
    "Targets other than `x86_64-unknown-uefi` are not supported",
    "\n",
    "Are you using `cargo build`? Try `cargo kbuild` instead."
));

use embedded_graphics::{
    mono_font::MonoTextStyle,
    pixelcolor::Rgb888,
    prelude::{DrawTarget, Point, RgbColor},
    text::Text,
    Drawable,
};
use profont::PROFONT_24_POINT;
use uefi::prelude::*;

pub mod framebuffer;

#[entry]
fn main(_handle: Handle, mut system_table: SystemTable<Boot>) -> Status {
    let mut fb = framebuffer::init(&mut system_table).unwrap();
    fb.clear(Rgb888::BLACK).unwrap();

    // Create a new character style
    let style = MonoTextStyle::new(&PROFONT_24_POINT, Rgb888::WHITE);

    // Create a text at position (20, 30) and draw it using the previously defined style
    Text::new("Hello, OS!", Point::new(15, 30), style)
        .draw(&mut fb)
        .unwrap();

    #[allow(clippy::empty_loop)]
    loop {}
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    log::error!("{info}");
    loop {}
}
