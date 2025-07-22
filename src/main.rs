#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(shos::test_runner)]
#![reexport_test_harness_main = "test_main"]

use::shos::println;

mod serial;

use core::panic::PanicInfo;

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
    use core::fmt::Write;
    println!("Welcome to shmageOS!");
    println!("           _");
    println!("        ___");
    println!("      l..l.l");
    println!("    __________");
    println!("  ______________");
    println!("_____________________");
    println!("ooooooooooooooooooooooo");
    println!("   |  =    =  |");
    println!("   j  O    O  j");
    println!(r"   \          /");
    println!("\"Writing a computer program is simple,");
    println!("but writing a simple computer program");
    println!("is the hardest thing there is!\" - Shawn");
    println!("_______________________");
    write!(shos::vga_buffer::WRITER.lock(), "Currently running ver {}", 1.0/3.0).unwrap();
    println!("");
    println!("_______________________");
    println!("");
    serial_println!("Hello from the serial device!");
    // You can also panic at any point!
    //panic!("uh oh!");
    #[cfg(test)]
    test_main();
    loop{}
}
