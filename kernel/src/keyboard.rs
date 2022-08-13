use crate::late_init::LateInit;
use pc_keyboard::{layouts, DecodedKey, Error, HandleControl, KeyEvent, Keyboard, ScancodeSet1};
use spin::Mutex;
use x86_64::instructions::port::Port;

pub static KEYBOARD: LateInit<Mutex<KeyboardLayout>> = LateInit::new();

pub enum KeyboardLayout {
    Azerty(Keyboard<layouts::Azerty, ScancodeSet1>),
    Dvorak(Keyboard<layouts::Dvorak104Key, ScancodeSet1>),
    Qwerty(Keyboard<layouts::Us104Key, ScancodeSet1>),
}

impl KeyboardLayout {
    fn add_byte(&mut self, scancode: u8) -> Result<Option<KeyEvent>, Error> {
        match self {
            KeyboardLayout::Azerty(keyboard) => keyboard.add_byte(scancode),
            KeyboardLayout::Dvorak(keyboard) => keyboard.add_byte(scancode),
            KeyboardLayout::Qwerty(keyboard) => keyboard.add_byte(scancode),
        }
    }

    fn process_keyevent(&mut self, key_event: KeyEvent) -> Option<DecodedKey> {
        match self {
            KeyboardLayout::Azerty(keyboard) => keyboard.process_keyevent(key_event),
            KeyboardLayout::Dvorak(keyboard) => keyboard.process_keyevent(key_event),
            KeyboardLayout::Qwerty(keyboard) => keyboard.process_keyevent(key_event),
        }
    }

    fn from(name: &str) -> Option<Self> {
        match name {
            "azerty" => Some(KeyboardLayout::Azerty(Keyboard::new(
                layouts::Azerty,
                ScancodeSet1,
                HandleControl::MapLettersToUnicode,
            ))),
            "dvorak" => Some(KeyboardLayout::Dvorak(Keyboard::new(
                layouts::Dvorak104Key,
                ScancodeSet1,
                HandleControl::MapLettersToUnicode,
            ))),
            "qwerty" => Some(KeyboardLayout::Qwerty(Keyboard::new(
                layouts::Us104Key,
                ScancodeSet1,
                HandleControl::MapLettersToUnicode,
            ))),
            _ => None,
        }
    }
}

pub fn set_keyboard(layout: &str) -> bool {
    if let Some(keyboard) = KeyboardLayout::from(layout) {
        KEYBOARD.init(Mutex::new(keyboard));
        true
    } else {
        false
    }
}

pub fn init() {
    set_keyboard(option_env!("KEYBOARD_LAYOUT").unwrap_or("qwerty"));
}

pub fn read_scancode() -> u8 {
    let mut port = Port::new(0x60);
    let scancode = unsafe { port.read() };
    unsafe { port.write(0) };
    scancode
}

pub fn read() -> char {
    let mut keyboard = KEYBOARD.lock();

    loop {
        let scancode = read_scancode();
        if let Ok(Some(key_event)) = keyboard.add_byte(scancode) &&
                let Some(DecodedKey::Unicode(key)) = keyboard.process_keyevent(key_event) {
            break key;
        }
    }
}
