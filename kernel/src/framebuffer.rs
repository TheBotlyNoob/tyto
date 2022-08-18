use embedded_graphics::{pixelcolor::Rgb888, prelude::*, Pixel};
use spin::Mutex;
use uefi::{
    prelude::*,
    proto::console::gop::{GraphicsOutput, Mode, ModeInfo},
};

use crate::late_init::LateInit;

pub static FRAMEBUFFER: LateInit<Mutex<FrameBuffer>> = LateInit::new();

#[derive(Debug, Clone)]
pub struct FrameBuffer {
    pub info: ModeInfo,
    ptr: *mut u32,
}
unsafe impl Sync for FrameBuffer {}
unsafe impl Send for FrameBuffer {}

pub fn init(system_table: &mut SystemTable<Boot>) {
    FRAMEBUFFER.init(|| {
        let gop = unsafe {
            &mut *system_table
                .boot_services()
                .locate_protocol::<GraphicsOutput>()
                .expect("Graphics output protocol not found")
                .get()
        };

        let mode = set_mode(gop);

        let mut fb = FrameBuffer {
            info: *mode.info(),
            ptr: gop.frame_buffer().as_mut_ptr() as _,
        };

        let _ = fb.clear(Rgb888::BLACK);

        Mutex::new(fb)
    });
}

impl DrawTarget for FrameBuffer {
    type Color = Rgb888;
    // `ExampleDisplay` uses a framebuffer and doesn't need to communicate with the display
    // controller to draw pixel, which means that drawing operations can never fail. To reflect
    // this the type `Infallible` was chosen as the `Error` type.
    type Error = core::convert::Infallible;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        for Pixel(coord, color) in pixels.into_iter() {
            // Check if the pixel coordinates are out of bounds (negative or greater than
            // the resolution). `DrawTarget` implementation are required to discard
            // any out of bounds pixels without returning an error or causing a panic.
            let (max_x, max_y) = self.info.resolution();

            if coord.x < 0 || coord.x >= max_x as _ || coord.y < 0 || coord.y >= max_y as _ {
                continue;
            }

            // Calculate the index in the framebuffer.
            let index = (coord.x + coord.y * max_x as i32) as usize;
            unsafe {
                self.ptr.add(index).write(color.into_storage());
            }
        }

        Ok(())
    }
}

impl OriginDimensions for FrameBuffer {
    fn size(&self) -> Size {
        Size::new(self.info.resolution().0 as _, self.info.resolution().1 as _)
    }
}

fn set_mode(gop: &mut GraphicsOutput) -> Mode {
    let mut current_mode = Option::<Mode>::None;
    for mode in gop.modes() {
        let resolution = mode.info().resolution();
        let current_resolution = current_mode.as_ref().map(|m| m.info().resolution());
        if current_resolution.map(|r| r < resolution).unwrap_or(true) {
            current_mode.replace(mode);
        }
    }

    let mode = current_mode.expect("No valid graphics mode found");

    let _ = gop.set_mode(&mode);

    mode
}
