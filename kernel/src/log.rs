use core::fmt::Write;

use crate::{framebuffer::FRAMEBUFFER, late_init::LateInit};
use embedded_graphics::{mono_font::MonoTextStyle, pixelcolor::Rgb888, prelude::*, text::Text};
use spin::Mutex;
use uart_16550::SerialPort;

pub static SERIAL1: Mutex<SerialPort> = Mutex::new(unsafe { SerialPort::new(0x3f8) });
pub static LOGGER: LateInit<Mutex<Logger>> = LateInit::new();

pub struct Logger {
    next_char: Point,
}
impl Logger {
    pub fn add_newline(&mut self) {
        self.next_char = Point::new(15, self.next_char.y + 26);
    }
    pub fn is_overflowing(&self) -> bool {
        self.next_char.x > FRAMEBUFFER.lock().info.resolution().0 as i32 - 20
    }
}

pub fn init() {
    SERIAL1.lock().init();
    LOGGER.init(|| {
        Mutex::new(Logger {
            next_char: Point::new(15, 30),
        })
    });
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
                    unsafe { core::str::from_utf8_unchecked(&[char as u8]) },
                    self.next_char,
                    MonoTextStyle::new(&profont::PROFONT_24_POINT, Rgb888::WHITE),
                )
                .draw(&mut *FRAMEBUFFER.lock())
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
    ($($arg:tt)*) => ($crate::log::_print(format_args!($($arg)*)));
}
#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[test_case]
fn test_println_simple() {
    println!("test_println_simple output");
}

#[test_case]
fn test_println_many() {
    for _ in 0..200 {
        println!("test_println_many output");
    }
}

#[test_case]
fn test_print_wrap() {
    for _ in 0..200 {
        print!("test_println_wrap output that should wrap");
    }
}
