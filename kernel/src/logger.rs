use core::fmt::Write;

use embedded_graphics::{
    draw_target::DrawTarget,
    mono_font::MonoTextStyle,
    pixelcolor::Rgb888,
    prelude::{Point, RgbColor},
    text::{renderer::TextRenderer, Baseline, Text},
    Drawable,
};
use log::Log;
use spin::Mutex;
use uart_16550::SerialPort;

use crate::fb::FRAMEBUFFER;

static mut SERIAL_PORT: SerialPort = unsafe { SerialPort::new(0x03F8) };

pub fn init() {
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

pub static TEXT_WRITER: Mutex<TextWriter> = Mutex::new(TextWriter::new());

pub struct TextWriter(Point);
impl TextWriter {
    const fn new() -> Self {
        Self(Point::new(5, 20))
    }
}
impl Write for TextWriter {
    #[allow(clippy::cast_sign_loss)]
    fn write_char(&mut self, c: char) -> core::fmt::Result {
        let string = [c as u8];
        // SAFETY: We can assume it's a valid char.
        let string = unsafe { core::str::from_utf8_unchecked(&string) };
        let style = MonoTextStyle::new(&profont::PROFONT_18_POINT, Rgb888::WHITE);

        if c == '\n'
            || style
                .measure_string(string, self.0, Baseline::Bottom)
                .next_position
                .x as usize
                > FRAMEBUFFER.info.width
        {
            self.0 = Point::new(5, self.0.y + 26);
        }

        let fb = &mut *FRAMEBUFFER.fb.lock();
        if self.0.y as usize > FRAMEBUFFER.info.height {
            fb.clear(Rgb888::BLACK).unwrap();
            self.0 = Point::new(5, 20);
        }

        self.0 = Text::new(string, self.0, style).draw(fb).unwrap();

        Ok(())
    }

    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for c in s.chars() {
            self.write_char(c)?;
        }

        Ok(())
    }
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
    let _ = write!(TEXT_WRITER.lock(), $($arg)*);
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
