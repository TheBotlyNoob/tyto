use core::fmt::Write;

use crate::{data::late_init::LateInit, framebuffer::FrameBuffer};
use embedded_graphics::{mono_font::MonoTextStyle, pixelcolor::Rgb888, prelude::*, text::Text};
use spin::Mutex;
use uart_16550::SerialPort;

pub static SERIAL1: Mutex<SerialPort> = Mutex::new(unsafe { SerialPort::new(0x3f8) });
pub static LOGGER: LateInit<Mutex<Logger>> = LateInit::new();

pub struct Logger {
    #[allow(dead_code)]
    framebuffer: FrameBuffer,
    next_char: Point,
}
impl Logger {
    pub fn init(framebuffer: FrameBuffer) {
        SERIAL1.lock().init();
        LOGGER.init(Mutex::new(Logger {
            framebuffer,
            next_char: Point::new(15, 30),
        }));
    }
}
impl Write for Logger {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        SERIAL1.lock().write_str(s)?;
        self.next_char = Text::new(
            s,
            self.next_char,
            MonoTextStyle::new(&profont::PROFONT_24_POINT, Rgb888::WHITE),
        )
        .draw(&mut self.framebuffer)
        .unwrap();

        Ok(())
    }
}

#[doc(hidden)]
pub fn _print(args: core::fmt::Arguments) {
    LOGGER.lock().write_fmt(args).unwrap();
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {
        $crate::log::_print(format_args!($($arg)*));
    };
}
#[macro_export]
macro_rules! println {
    ($($arg:tt)*) => {
        print!("{}\n", format_args!($($arg)*));
    };
}
