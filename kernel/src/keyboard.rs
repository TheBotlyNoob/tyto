// TODO: move to a USB controller
use ps2::{error::ControllerError, flags::ControllerConfigFlags, Controller};
use spin::{Lazy, Mutex};

pub static CONTROLLER: Lazy<Mutex<Controller>> = Lazy::new(|| {
    (|| -> Result<Mutex<Controller>, ControllerError> {
        let mut controller = unsafe { Controller::new() };

        // Step 3: Disable devices
        controller.disable_keyboard()?;
        controller.disable_mouse()?;

        // Step 4: Flush data buffer
        let _ = controller.read_data();

        // Step 5: Set config
        let mut config = controller.read_config()?;
        // Disable interrupts and scancode translation
        config.set(ControllerConfigFlags::ENABLE_TRANSLATE, false);
        controller.write_config(config)?;

        // Step 6: Controller self-test
        controller.test_controller()?;
        // Write config again in case of controller reset
        controller.write_config(config)?;

        // Step 8: Interface tests
        let keyboard_works = controller.test_keyboard().is_ok();

        // Step 9 - 10: Enable and reset devices
        config = controller.read_config()?;
        if keyboard_works {
            controller.enable_keyboard()?;
            config.set(ControllerConfigFlags::DISABLE_KEYBOARD, false);
            config.set(ControllerConfigFlags::ENABLE_KEYBOARD_INTERRUPT, true);
            controller.keyboard().reset_and_self_test().unwrap();
        }

        Ok(Mutex::new(controller))
    })()
    .unwrap()
});

static SCAN_CODES_TO_ASCII: phf::Map<u8, char> = phf::phf_map! {
    0x1C_u8 => 'A',
    0x32_u8 => 'B',
    0x21_u8 => 'C',
    0x23_u8 => 'D',
    0x24_u8 => 'E',
    0x2B_u8 => 'F',
    0x34_u8 => 'G',
    0x33_u8 => 'H',
    0x43_u8 => 'I',
    0x3B_u8 => 'J',
    0x42_u8 => 'K',
    0x4B_u8 => 'L',
    0x3A_u8 => 'M',
    0x31_u8 => 'N',
    0x44_u8 => 'O',
    0x4D_u8 => 'P',
    0x15_u8 => 'Q',
    0x2D_u8 => 'R',
    0x1B_u8 => 'S',
    0x2C_u8 => 'T',
    0x3C_u8 => 'U',
    0x2A_u8 => 'V',
    0x1D_u8 => 'W',
    0x22_u8 => 'X',
    0x35_u8 => 'Y',
    0x1A_u8 => 'Z',
    0x45_u8 => '0',
    0x16_u8 => '1',
    0x1E_u8 => '2',
    0x26_u8 => '3',
    0x25_u8 => '4',
    0x2E_u8 => '5',
    0x36_u8 => '6',
    0x3D_u8 => '7',
    0x3E_u8 => '8',
    0x46_u8 => '9',

};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Keyboard;

impl Keyboard {
    /// Waits for a keypress and returns the keycode.
    pub fn next_char() -> u8 {
        let mut controller = CONTROLLER.lock();
        loop {
            if let Ok(c) = controller.read_data() && c != 240 {
                return *SCAN_CODES_TO_ASCII.get(&c).unwrap_or(&'?') as u8;
            }
        }
    }
}
