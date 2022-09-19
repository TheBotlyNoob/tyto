use core::{cell::UnsafeCell, convert::Infallible};

use crate::late_init::LateInit;
use bootloader_api::info::{FrameBuffer as InnerFrameBuffer, PixelFormat};
use embedded_graphics::{
    pixelcolor::{Gray8, Rgb888},
    prelude::{DrawTarget, GrayColor, IntoStorage, OriginDimensions, Size},
    Pixel,
};

pub struct FrameBufferWriter<'a>(&'a mut InnerFrameBuffer);
impl DrawTarget for FrameBufferWriter<'_> {
    type Color = Rgb888;
    type Error = Infallible;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = embedded_graphics::Pixel<Self::Color>>,
    {
        log::info!("{:#?}", self.0.info());
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
impl OriginDimensions for FrameBufferWriter<'_> {
    fn size(&self) -> Size {
        Size::new(self.0.info().width as u32, self.0.info().height as u32)
    }
}

pub struct FrameBuffer(LateInit<UnsafeCell<InnerFrameBuffer>>);
unsafe impl Sync for FrameBuffer {}
unsafe impl Send for FrameBuffer {}
impl FrameBuffer {
    pub const fn new() -> Self {
        Self(LateInit::new())
    }
    pub fn init(&self, framebuffer: InnerFrameBuffer) {
        self.0.init(UnsafeCell::new(framebuffer))
    }
    pub fn get(&self) -> FrameBufferWriter<'_> {
        FrameBufferWriter(unsafe { &mut *self.0.get().get() })
    }
}

pub static FRAMEBUFFER: FrameBuffer = FrameBuffer::new();

pub fn init_framebuffer(fb: Option<InnerFrameBuffer>) {
    let mut fb = fb.expect("no framebuffer");
    fb.buffer_mut().fill(0);

    if matches!(fb.info().pixel_format, PixelFormat::Unknown { .. }) {
        panic!("framebuffer has unsupported pixel format");
    }

    FRAMEBUFFER.init(fb);
}
