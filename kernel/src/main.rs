#![no_main]
#![no_std]
#![feature(abi_efiapi)]

use embedded_graphics::{
    pixelcolor::Rgb888,
    prelude::{DrawTarget, RgbColor},
};
use uefi::prelude::*;

pub mod framebuffer;

// #[global_allocator]
// static ALLOCATOR: linked_list_allocator::LockedHeap = linked_list_allocator::LockedHeap::empty();

#[entry]
fn main(_handle: Handle, mut system_table: SystemTable<Boot>) -> Status {
    uefi_services::init(&mut system_table).unwrap();

    let mut fb = framebuffer::init(&mut system_table).unwrap();
    fb.clear(Rgb888::BLACK).unwrap();

    #[allow(clippy::empty_loop)]
    loop {}
}

// #[panic_handler]
// fn panic(_info: &core::panic::PanicInfo) -> ! {
//     loop {}
// }
