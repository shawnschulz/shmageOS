#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(shos::test_runner)]
#![reexport_test_harness_main = "test_main"]

mod serial;

use core::panic::PanicInfo;
use shos::{memory::translate_address, println};
use x86_64::{registers::control::Cr3Flags, structures::paging::PageTable};
use::bootloader::{BootInfo, entry_point};

/// This function is called on panic.
#[cfg(not(test))] // new attribute
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {
        x86_64::instructions::hlt();
    }
}

fn kernel_main (boot_info: &'static BootInfo) -> ! {
    use shos::memory::translate_address;
    use x86_64::VirtAddr;
    shos::init();
    shos::shfetch();
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
    use shos::print;
    #[cfg(test)]
    test_main();
    shos::hlt_loop();
}

entry_point!(kernel_main);

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    shos::test_panic_handler(info)
}
