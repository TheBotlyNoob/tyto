pub fn halt() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    crate::println!("{info}");
    #[cfg(test)]
    crate::test::exit_qemu(test::QemuExitCode::Failed);
    halt();
}
