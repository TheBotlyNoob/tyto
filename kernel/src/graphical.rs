use bootloader_api::info::{FrameBuffer, PixelFormat};

use crate::late_init::LateInit;

pub static FRAMEBUFFER: LateInit<FrameBuffer> = LateInit::new();

pub fn init_framebuffer(fb: Option<FrameBuffer>) {
    let mut fb = fb.expect("no framebuffer");
    fb.buffer_mut().fill(0);
    if matches!(fb.info().pixel_format, PixelFormat::Unknown { .. }) {
        panic!("framebuffer has unknown pixel format");
    }
    FRAMEBUFFER.init(fb);
}
