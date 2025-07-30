#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(shos::test_runner)]
#![reexport_test_harness_main = "test_main"]

mod serial;

use core::panic::PanicInfo;
use shos::println;
use x86_64::registers::control::Cr3Flags;

/// This function is called on panic.
#[cfg(not(test))] // new attribute
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {
        x86_64::instructions::hlt();
    }
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

    shos::shfetch();
    use x86_64::registers::control::Cr3;
    let (phys_frame, flags) = Cr3::read();
    println!("Level 4 page table at: {:?}", phys_frame.start_address());
    println!("Flags: {:?}", flags);
    // invoke a breakpoint exception
    // You can also panic at any point!
    //panic!("uh oh!");
    #[cfg(test)]
    test_main();
    use shos::print;
    loop{
        x86_64::instructions::hlt();
    }
}
