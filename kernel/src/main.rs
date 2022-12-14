#![no_main]
#![no_std]
#![feature(
    abi_efiapi,
    abi_x86_interrupt,
    custom_test_frameworks,
    alloc_error_handler,
    if_let_guard
)]
#![test_runner(crate::tests::test_runner)]
#![reexport_test_harness_main = "test_main"]

#[cfg(not(all(target_arch = "x86_64", target_vendor = "unknown", target_os = "uefi")))]
compile_error!(concat!(
    "Targets other than `x86_64-unknown-uefi` are not supported",
    "\n",
    "Are you using `cargo build`? Try `cargo kbuild` instead."
));

extern crate alloc;

use alloc::string::String;
use uefi::prelude::*;

pub mod framebuffer;
pub mod keyboard;
pub mod late_init;
pub mod log;
pub mod util;

#[cfg(test)]
pub mod test;

#[global_allocator]
static ALLOCATOR: static_alloc::Bump<[u8; 4 << 16]> = static_alloc::Bump::uninit();

#[entry]
pub fn main(_handle: Handle, mut system_table: SystemTable<Boot>) -> Status {
    framebuffer::init(&mut system_table);
    log::init();

    #[cfg(test)]
    {
        test_main();
        util::halt();
    }

    let mut input = String::new();
    loop {
        print!("> ");
        input.clear();
        keyboard::read_line(&mut input);

        match input.as_str() {
            "" => println!(),
            _ if let Some(echo) = input.strip_prefix("echo ") => println!("\n{echo}"),
            "exit" => break,
            _ => println!("\nUnknown command: {input}"),
        }
    }

    util::halt();
}
