#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(walnut_os::testing::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
pub use walnut_os::testing::test_runner;
use walnut_os::{println, serial_println};

#[allow(unconditional_panic)]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    serial_println!("Walnut Initializing");
    println!("Walnut Initializing");
    walnut_os::init();

    fn stack_overflow() {
        stack_overflow(); // for each recursion, the return address is pushed
    }

    // trigger a stack overflow
    stack_overflow();

    serial_println!("Entering OS loop");
    #[cfg(test)]
    test_main();
    loop {}
}

/// This function is called on panic.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    serial_println!("PANIC: {}", info);
    loop {}
}
