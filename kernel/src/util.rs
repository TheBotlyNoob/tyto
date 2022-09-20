use x86_64::instructions::hlt;

pub fn halt() -> ! {
    loop {
        hlt();
    }
}
