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

#[test_case]
fn test_late_init() {
    let mut x = LateInit::<i32>::new();
    x.init(42);
    assert_eq!(*x, 42);
}

#[test_case]
fn test_late_init_mutability() {
    let mut x = LateInit::<i32>::new();
    x.init(42);

    *x = 43;

    assert_eq!(*x, 43);
}
