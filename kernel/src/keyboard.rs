use alloc::string::String;
use pc_keyboard::{layouts, DecodedKey, HandleControl, Keyboard, ScancodeSet1};
use spin::Mutex;
use x86_64::instructions::port::Port;

pub static KEYBOARD: Mutex<Keyboard<layouts::Us104Key, ScancodeSet1>> = Mutex::new(Keyboard::new(
    layouts::Us104Key,
    ScancodeSet1,
    HandleControl::MapLettersToUnicode,
));

fn read_scancode() -> u8 {
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

pub fn read_line(s: &mut String) {
    loop {
        let key = read();
        if key == '\n' {
            break;
        } else if key == '\x08' {
            s.pop();
        } else {
            s.push(key);
        }
        crate::print!("{key}");
    }
}
