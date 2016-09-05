use ffi;
use core::Option;

pub struct Vec<T> {
    _incomplete: [T; 0],
};

impl Vec {
    pub fn new() -> Vec {
        js!("return []");

        // Fake value for rustc.
        Vec {
            _incomplete: [],
        }
    }

    pub fn push(&mut self, elem: T) {
        js!("a0.push(a1)")
    }

    pub fn pop(&mut self) -> Option<T> {
        let res = js!("a0.pop()");

        if res == ffi::undefined() {
            Option::None
        } else {
            Option::Some(res)
        }
    }
}
