pub enum Option<T> {
    Some(T),
    None,
}

#[macro_export]
macro_rules! unreachable {
    () => {
        js!("alert('Cyano error: A codepath marked unreachable was reached.')");

        loop {}
    };
}

/* TODO

#[lang = "eh_personality"]
#[no_mangle]
pub extern fn rust_eh_personality() {}

#[lang = "eh_unwind_resume"]
#[no_mangle]
pub extern fn rust_eh_unwind_resume() {}

#[lang = "panic_fmt"]
#[no_mangle]
pub extern fn rust_begin_panic(_msg: core::fmt::Arguments, _file: &'static str, _line: u32) -> ! {
    // TODO: Give the message here.
    js!("alert('Panic!')");

    loop {}
}

*/
