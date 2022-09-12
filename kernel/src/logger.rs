use core::fmt::Write;

use bootloader_api::info::FrameBuffer;
use uart_16550::SerialPort;

use crate::graphical;

#[derive(Clone, Copy)]
pub enum Color {
    White,
    Red,
    Green,
    Blue,
    Yellow,
}

static mut SERIAL_PORT: SerialPort = unsafe { SerialPort::new(0x03F8) };

pub fn init(framebuffer: Option<FrameBuffer>) {
    graphical::init_framebuffer(framebuffer);
}

pub struct Logger;

impl log::Log for Logger {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        metadata.level() <= log::Level::Info
    }

    fn log(&self, record: &log::Record) {
        let color = match record.level() {
            log::Level::Error => Color::Red,
            log::Level::Warn => Color::Yellow,
            log::Level::Info => Color::White,
            log::Level::Debug => Color::Blue,
            log::Level::Trace => Color::Green,
        };

        /// the actual logging implementation
        fn _log(s: core::fmt::Arguments, color: Color) {
            graphical::write(&s, color);

            unsafe {
                write!(SERIAL_PORT, "{}{s}\r\n", color.escape_sequence()).unwrap();
            }
        }

        _log(
            format_args!("[{}] {}", record.level(), record.args()),
            color,
        );
    }

    fn flush(&self) {}
}

impl Color {
    fn escape_sequence(&self) -> &'static str {
        match self {
            Color::White => "\x1b[m\x1b[97m",
            Color::Red => "\x1b[m\x1b[91m",
            Color::Green => "\x1b[m\x1b[92m",
            Color::Blue => "\x1b[m\x1b[94m",
            Color::Yellow => "\x1b[m\x1b[93m",
        }
    }
}
