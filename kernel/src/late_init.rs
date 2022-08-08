use core::ops::{Deref, DerefMut};

use spin::Once;

pub struct LateInit<T>(Once<T>);
impl<T> LateInit<T> {
    pub const fn new() -> Self {
        Self(Once::new())
    }
    pub fn init(&self, x: T) {
        self.0.call_once(|| x);
    }

    pub fn get(&self) -> &T {
        self.0.get().expect("LateInit not initialized")
    }
    pub fn get_mut(&mut self) -> &mut T {
        self.0.get_mut().expect("LateInit not initialized")
    }
}

impl<T> Deref for LateInit<T> {
    type Target = T;
    fn deref(&self) -> &T {
        self.get()
    }
}
impl<T> DerefMut for LateInit<T> {
    fn deref_mut(&mut self) -> &mut T {
        self.get_mut()
    }
}
