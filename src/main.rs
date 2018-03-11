#![feature(lang_items)]
#![no_main]
#![no_std]

#[lang = "panic_fmt"]
#[no_mangle]
pub extern fn rust_begin_panic(_msg: core::fmt::Arguments,
    _file: &'static str, _line: u32, _column: u32) -> ! {
    loop {}
}

#[no_mangle]
pub extern fn _start() -> ! {
    loop {}
}
