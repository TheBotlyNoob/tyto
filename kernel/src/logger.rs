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
        println!("[{}] {}", record.level(), record.args());
    }

    fn flush(&self) {}
}

/// A macro to print to both the serial port and the framebuffer with a newline.
pub macro println {
    () => {
        print!("\n");
    },
    ($($arg:tt)*) => {
        print!("{}\n", format_args!($($arg)*));
    }
}
/// A macro to print to both the serial port and the framebuffer.
pub macro print($($arg:tt)*) {
    sprint!($($arg)*);
    let _ = write!(crate::fb::TEXT_WRITER.lock(), $($arg)*);
}

/// A macro to print to the serial port with a newline.
pub macro sprintln {
    () => {
        sprint!("\n")
    },
    ($($arg:tt)*)  =>{
        sprint!("{}\n", format_args!($($arg)*));
    }
}
/// A macro to print to the serial port.
pub macro sprint($($arg:tt)*) {
    unsafe {
        let _ = write!(SERIAL_PORT, $($arg)*);
    }
}
