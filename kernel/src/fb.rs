use crate::{
    data::LateInit,
    logger::{sprint, sprintln},
};
use bootloader_api::info::{FrameBuffer as InnerFrameBuffer, FrameBufferInfo, PixelFormat};
use core::convert::Infallible;
use embedded_graphics::{
    pixelcolor::{Gray8, Rgb888},
    prelude::*,
    Pixel,
};
use spin::Mutex;

// TODO(@TheBotlyNoob): Fix in BIOS mode

pub struct FrameBufferWriter(InnerFrameBuffer);
impl DrawTarget for FrameBufferWriter {
    type Color = Rgb888;
    type Error = Infallible;

    #[allow(
        clippy::cast_possible_truncation,
        clippy::cast_possible_wrap,
        clippy::cast_sign_loss
    )]
    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = embedded_graphics::Pixel<Self::Color>>,
    {
        let info = self.0.info();
        sprintln!("{info:#?}");

        for Pixel(coord, color) in pixels {
            // sleep for 0.5 seconds
            for _ in 0..100_000 {
                unsafe { core::arch::asm!("nop") };
            }
            // Calculate the index in the framebuffer.
            let index = (coord.x + coord.y * info.stride as i32) as usize;

            macro buf {
                (u32, $x:expr) => {
                    buf!(unsafe {
                        &mut *core::ptr::slice_from_raw_parts_mut(
                            (self.0.buffer_mut() as *mut [u8]).cast::<u32>(),
                            self.0.buffer().len() / 4,
                        )
                    }, $x)
                },
                (u8, $x:expr) => {
                    buf!(self.0.buffer_mut(), $x)
                },
                ($buf:expr, $x:expr) => {{
                    $buf.get_mut(index).map(|x| *x = $x);
                }},
            }
            match info.pixel_format {
                PixelFormat::Rgb => buf!(u32, color.into_storage()),
                PixelFormat::Bgr => buf!(u32, color.into_storage().swap_bytes()),
                PixelFormat::U8 => buf!(u8, Gray8::from(color).luma()),
                _ => unimplemented!(),
            };
        }

        Ok(())
    }
}
impl OriginDimensions for FrameBufferWriter {
    #[allow(clippy::cast_possible_truncation)]
    fn size(&self) -> Size {
        Size::new(self.0.info().width as u32, self.0.info().height as u32)
    }
}

pub struct FrameBuffer {
    pub fb: Mutex<FrameBufferWriter>,
    pub info: FrameBufferInfo,
}

pub static FRAMEBUFFER: LateInit<FrameBuffer> = LateInit::new();

pub fn init_framebuffer(fb: Option<InnerFrameBuffer>) {
    let mut fb = fb.expect("no framebuffer");
    fb.buffer_mut().fill(0);

    FRAMEBUFFER.init(FrameBuffer {
        info: fb.info(),
        fb: Mutex::new(FrameBufferWriter(fb)),
    });
}
