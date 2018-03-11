#![feature(lang_items)]
#![no_main]
#![no_std]

// Prevent linker faults because functions normally provided by libc aren't
// there. LLVM lowers some intrinsics which will now keep working happily.
extern crate rlibc;

/// Raw pointer to the VGA buffer start. The VGA buffer is available via memory
/// mapped IO as `0xb8000`.
/// See more about raw pointers at
/// https://doc.rust-lang.org/stable/book/second-edition/ch19-01-unsafe-rust.html#dereferencing-a-raw-pointer
const VGA_BUFFER_ADDR: *mut u8 = 0xb8000 as *mut u8;
static BOOT_MESSAGE: &[u8] = b"ROS: Rust Operating System";

static COLOUR_LIGHT_GREEN: u8 = 0xc;

/// Initial entrypoint for kernel.
#[no_mangle]
pub extern fn _start() -> ! {
    for (i, &byte) in BOOT_MESSAGE.iter().enumerate() {
        unsafe {
            *VGA_BUFFER_ADDR.offset(i as isize * 2) = byte;
            *VGA_BUFFER_ADDR.offset(i as isize * 2 + 1) = COLOUR_LIGHT_GREEN;
        }
    }

    loop {}
}


/// Panic handler.
#[lang = "panic_fmt"]
#[no_mangle]
pub extern fn rust_begin_panic(_msg: core::fmt::Arguments,
    _file: &'static str, _line: u32, _column: u32) -> ! {
    loop {}
}
