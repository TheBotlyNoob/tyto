use crate::late_init::LateInit;
use bootloader_api::info::{FrameBuffer as InnerFrameBuffer, FrameBufferInfo, PixelFormat};
use core::{convert::Infallible, fmt::Write};
use embedded_graphics::{
    mono_font::MonoTextStyle,
    pixelcolor::{Gray8, Rgb888},
    prelude::*,
    text::{renderer::TextRenderer, Baseline, Text},
    Pixel,
};
use spin::Mutex;

pub struct FrameBufferWriter(InnerFrameBuffer);
impl DrawTarget for FrameBufferWriter {
    type Color = Rgb888;
    type Error = Infallible;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = embedded_graphics::Pixel<Self::Color>>,
    {
        for Pixel(coord, color) in pixels.into_iter() {
            let info = self.0.info();
            // Check if the pixel coordinates are out of bounds (negative or greater than
            // the resolution). `DrawTarget` implementation are required to discard
            // any out of bounds pixels without returning an error or causing a panic.
            let (max_x, max_y) = (self.0.info().width, self.0.info().height);

            if coord.x < 0 || coord.x >= max_x as _ || coord.y < 0 || coord.y >= max_y as _ {
                continue;
            }
            // Calculate the index in the framebuffer.
            let index = (coord.x + coord.y * info.stride as i32) as usize;
            let buf = unsafe { core::mem::transmute::<_, &mut [u32]>(self.0.buffer_mut()) };
            match info.pixel_format {
                PixelFormat::Rgb => buf[index] = color.into_storage(),
                PixelFormat::Bgr => buf[index] = color.into_storage().swap_bytes(),
                // Using the normal buffer because there will only be one u8 per pixel
                PixelFormat::U8 => self.0.buffer_mut()[index] = Gray8::from(color).luma(),
                _ => unimplemented!(),
            }
        }

        Ok(())
    }
}
impl OriginDimensions for FrameBufferWriter {
    fn size(&self) -> Size {
        Size::new(self.0.info().width as u32, self.0.info().height as u32)
    }
}

pub struct FrameBuffer {
    fb: Mutex<FrameBufferWriter>,
    info: FrameBufferInfo,
}
pub static FRAMEBUFFER: LateInit<FrameBuffer> = LateInit::new();

pub fn init_framebuffer(fb: Option<InnerFrameBuffer>) {
    let mut fb = fb.expect("no framebuffer");
    fb.buffer_mut().fill(0);

    if matches!(fb.info().pixel_format, PixelFormat::Unknown { .. }) {
        panic!("framebuffer has unsupported pixel format");
    }

    FRAMEBUFFER.init(FrameBuffer {
        info: fb.info(),
        fb: Mutex::new(FrameBufferWriter(fb)),
    });
}

pub static TEXT_WRITER: Mutex<TextWriter> = Mutex::new(TextWriter::new());

pub struct TextWriter(Point);
impl TextWriter {
    const fn new() -> Self {
        Self(Point::new(5, 20))
    }
}
impl Write for TextWriter {
    fn write_char(&mut self, c: char) -> core::fmt::Result {
        let string = [c as u8];
        // SAFETY: The char comes from a string.
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
