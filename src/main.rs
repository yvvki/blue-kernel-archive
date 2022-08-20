#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points

#[no_mangle] // don't mangle the name of this function
pub extern "C" fn _start() -> ! {
    // This function is the entry point, since the linker looks for a function
    // named `_start` by default.
    loop {}
}

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
