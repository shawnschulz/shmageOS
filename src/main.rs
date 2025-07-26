#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(shos::test_runner)]
#![reexport_test_harness_main = "test_main"]

mod serial;

use core::panic::PanicInfo;
use shos::println;

/// This function is called on panic.
#[cfg(not(test))] // new attribute
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    shos::test_panic_handler(info)
}

// This removes name mangling so compiler
// doesn't try to link to C std library
#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    shos::init();
    // invoke a breakpoint exception
    shos::shfetch();
    // You can also panic at any point!
    //panic!("uh oh!");
    #[cfg(test)]
    test_main();
    loop{}
}
