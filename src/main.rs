#![feature(const_fn)]
#![feature(lang_items)]
#![no_main]
#![no_std]

// Enables us to work around the fact that that Rust's const evaluator is not
// able to convert raw pointers to references at compile time.
#[macro_use]
extern crate lazy_static;

// Prevent linker faults because functions normally provided by libc aren't
// there. LLVM lowers some intrinsics which will now keep working happily.
extern crate rlibc;

// Gives us spinlocks, where we basically spin the CPU until the lock is free.
// Why this and not a Mutex or something? Because we're in an OS so we have
// no threads or even the concept of blocking!
extern crate spin;

// Allows us to mark regions as volatile and therefore off limits for
// optimisation.
extern crate volatile;

#[macro_use]
mod vga_buffer;

static BOOT_MESSAGE: &str = "ROS: Rust Operating System";

/// Initial entrypoint for kernel.
#[no_mangle]
pub extern fn _start() -> ! {
    println!("{}", BOOT_MESSAGE);
    println!("We are now able to print multiple lines to the buffer!");

    loop {}
}


/// Panic handler.
#[lang = "panic_fmt"]
#[no_mangle]
pub extern fn rust_begin_panic(_msg: core::fmt::Arguments,
    _file: &'static str, _line: u32, _column: u32) -> ! {
    loop {}
}
