#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]


#[cfg(test)]
pub fn test_runner(tests: &[&dyn Fn()]) {
    println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }
}

mod vga_buffer;

use core::panic::PanicInfo;

/// This function is called on panic.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
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
    write!(vga_buffer::WRITER.lock(), "Currently running ver {}", 1.0/3.0).unwrap();
    println!("");
    println!("_______________________");
    println!("");
    // You can also panic at any point!
    // panic!("uh oh!");
    #[cfg(test)]
    test_main();
    loop{}
}

#[test_case]
fn trivial_assertion() {
    print!("trivial assertion... ");
    assert_eq!(1, 1);
    println!("[ok]");
}
