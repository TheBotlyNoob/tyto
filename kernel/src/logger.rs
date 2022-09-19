use core::fmt::Write;

use bootloader_api::info::FrameBuffer;
use log::Log;
use uart_16550::SerialPort;

use crate::fb;

static mut SERIAL_PORT: SerialPort = unsafe { SerialPort::new(0x03F8) };

pub fn init(framebuffer: Option<FrameBuffer>) {
    fb::init_framebuffer(framebuffer);
    log::set_logger(&Logger).unwrap();
    log::set_max_level(log::LevelFilter::Trace);
}

pub struct Logger;

impl Log for Logger {
    fn enabled(&self, _metadata: &log::Metadata) -> bool {
        true
    }

    fn log(&self, record: &log::Record) {
        unsafe {
            let _ = write!(SERIAL_PORT, "[{}] {}\n", record.level(), record.args());
        }
    }

    fn flush(&self) {}
}
