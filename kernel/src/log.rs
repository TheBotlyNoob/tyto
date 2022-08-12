use core::fmt::Write;

use crate::{framebuffer::FrameBuffer, late_init::LateInit};
use embedded_graphics::{mono_font::MonoTextStyle, pixelcolor::Rgb888, prelude::*, text::Text};
use spin::Mutex;
use uart_16550::SerialPort;

pub static SERIAL1: Mutex<SerialPort> = Mutex::new(unsafe { SerialPort::new(0x3f8) });
pub static LOGGER: LateInit<Mutex<Logger>> = LateInit::new();

pub struct Logger {
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
    pub fn add_newline(&mut self) {
        self.next_char = Point::new(15, self.next_char.y + 26);
    }
    pub fn is_overflowing(&self) -> bool {
        self.next_char.x > self.framebuffer.info.resolution().0 as i32 - 20
    }
}
impl Write for Logger {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        SERIAL1.lock().write_str(s)?;

        for char in s.chars() {
            if char == '\n' || self.is_overflowing() {
                self.add_newline();
            } else {
                self.next_char = Text::new(
                    // SAFETY: The char comes from a string.
                    unsafe { core::str::from_utf8(&[char as u8]).unwrap_unchecked() },
                    self.next_char,
                    MonoTextStyle::new(&profont::PROFONT_24_POINT, Rgb888::WHITE),
                )
                .draw(&mut self.framebuffer)
                .unwrap();
            }
        }

        Ok(())
    }
}

#[doc(hidden)]
pub fn _print(args: core::fmt::Arguments) {
    let _ = LOGGER.lock().write_fmt(args);
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
        $crate::print!("{}\n", format_args!($($arg)*));
    };
}
