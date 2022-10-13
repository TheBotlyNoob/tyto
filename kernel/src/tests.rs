use core::hint::unreachable_unchecked;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11,
}

pub fn exit_qemu(exit_code: QemuExitCode) -> ! {
    use x86_64::instructions::port::Port;

    unsafe {
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }

    unsafe { unreachable_unchecked() };
}

#[cfg(test)]
pub fn test_runner(tests: &[&dyn Testable]) {
    log::info!("Running {} tests", tests.len());
    for test in tests {
        test.run();
    }
    exit_qemu(QemuExitCode::Success);
}

pub trait Testable {
    fn run(&self);
}

impl<T: FnOnce()> Testable for T {
    fn run(&self) {
        log::info!("\n{}...\n", core::any::type_name::<T>());
        self();
        log::info!("\n[ok]");
    }
}
