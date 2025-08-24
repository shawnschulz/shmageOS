#![cfg_attr(test, no_main)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]
#![no_std]
// Allows us to use c x86_interrupt calling convention in rust even though
// the ABI is unstable
#![feature(abi_x86_interrupt)]

use core::panic::PanicInfo;
use memory::translate_address;
use x86_64::{registers::control::Cr3Flags, structures::paging::PageTable};
use::bootloader::{BootInfo, entry_point};

pub mod serial;
pub mod vga_buffer;
pub mod interrupts;
pub mod gdt;
pub mod memory;

// Handles everything immediately after booting,
// eventually may want to then have this load or link against
// a threaded daemon that handles more complex launch and service
// management logic
pub fn init() {
    gdt::init();
    interrupts::initialize_idt();
    unsafe { interrupts::PICS.lock().initialize() };
    x86_64::instructions::interrupts::enable();
}
pub fn hlt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}


pub fn shfetch() {
    use core::fmt::Write;
    println!("Welcome to shmageOS!");
    println!("           _              user@wip");
    println!("        ___               ------------------------------");
    println!("      l..l.l              OS: shmageOS 0.0.1 x86_64");
    println!("    __________            Host: QEMU");
    println!("  ______________          Kernel: 0.0.1");
    println!("_____________________     Cluster Connections:");
    println!("ooooooooooooooooooooooo   Network:");
    println!("   |  =    =  |           CPU:");
    println!("   j  O    O  j           GPU:");
    println!(r"   \          /           Mem:");
    println!("                          ------------------------------");
    println!("\"Writing a computer program is simple,");
    println!("but writing a simple computer program");
    println!("is the hardest thing there is!\" - Shawn");
    println!("_______________________");
    write!(vga_buffer::WRITER.lock(), "Currently running ver {}", 1.0/3.0).unwrap();
    println!("");
    println!("_______________________");
    println!("");
    serial_println!("Hello from the serial device!");
}


pub trait Testable {
    fn run(&self) -> ();
}

impl<T> Testable for T
where
    T: Fn(),
{
    fn run(&self) {
        serial_print!("{}...\t", core::any::type_name::<T>());
        self();
        serial_println!("[ok]");
    }
}

pub fn test_runner(tests: &[&dyn Testable]) {
    serial_println!("Running {} tests", tests.len());
    for test in tests {
        test.run();
    }
    exit_qemu(QemuExitCode::Success);
}

pub fn test_panic_handler(info: &PanicInfo) -> ! {
    serial_println!("[failed]\n");
    serial_println!("Error: {}\n", info);
    exit_qemu(QemuExitCode::Failed);
    loop {}
}

/// Entry point for `cargo test`
#[cfg(test)]
#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    init();
    test_main();
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    test_panic_handler(info)
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11,
}

pub fn exit_qemu(exit_code: QemuExitCode) {
    use x86_64::instructions::port::Port;

    unsafe {
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn abort() -> !{
    loop {
        // process waits for some interrupt indefinitely on abort
        use core::arch::asm;
        unsafe {asm!("wfi")};
    }
}

// replacing the eh_personality C function name
#[unsafe(no_mangle)]
pub extern "C" fn eh_personality() {}

/// This function is called on panic.
#[cfg(not(test))] // new attribute
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {
        x86_64::instructions::hlt();
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn kernel_main (boot_info: &'static BootInfo) -> ! {
    use memory::translate_address;
    use x86_64::VirtAddr;
    init();
    shfetch();
    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let addresses = [
        0xb8000,
        0x201008,
        0x0100_0020_1a10,
        boot_info.physical_memory_offset,
    ];
    for &address in &addresses {
        let virtual_address = VirtAddr::new(address);
        let physical_address = unsafe{ translate_address(virtual_address, phys_mem_offset) } ;
        println!("{:?} | {:?}", virtual_address, physical_address);
    };
    #[cfg(test)]
    test_main();
    hlt_loop();
}

entry_point!(kernel_main);

