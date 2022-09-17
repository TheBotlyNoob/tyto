use core::{cell::UnsafeCell, convert::Infallible};

use crate::late_init::LateInit;
use bootloader_api::info::{FrameBuffer as InnerFrameBuffer, PixelFormat};
use embedded_graphics::{
    pixelcolor::Rgb888,
    prelude::{DrawTarget, OriginDimensions, Size},
};

pub struct FrameBufferWriter<'a>(&'a mut InnerFrameBuffer);
impl DrawTarget for FrameBufferWriter<'_> {
    type Color = Rgb888;
    type Error = Infallible;

    fn draw_iter<I>(&mut self, _pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = embedded_graphics::Pixel<Self::Color>>,
    {
        todo!();
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
        panic!("framebuffer has unknown pixel format");
    }

    FRAMEBUFFER.init(fb);
}
