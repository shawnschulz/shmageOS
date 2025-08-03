#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(shos::test_runner)]
#![reexport_test_harness_main = "test_main"]

mod serial;

use core::panic::PanicInfo;
use shos::println;
use x86_64::registers::control::Cr3Flags;
use::bootloader_api::{BootInfo, entry_point};

/// This function is called on panic.
#[cfg(not(test))] // new attribute
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {
        x86_64::instructions::hlt();
    }
}

fn kernel_main (boot_info: &'static mut BootInfo) -> ! {
    use shos::memory::active_level_4_table;
    use x86_64::VirtAddr;
    shos::init();
    shos::shfetch();
    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset.into_option().unwrap());
    let l4_table = unsafe { active_level_4_table(phys_mem_offset) };

    for (i, entry) in l4_table.iter().enumerate() {
        if !entry.is_unused() {
            println!("L4 entry: {}: {:?}", i, entry);
            // get physical address from entry and convert
            let physal_address = entry.frame().unwrap().start_address();
            let virtual_address = physal_address.as_u64() + boot_info.physical_memory_offset.into_option().unwrap();
            let mut virtual_ptr = VirtAddr::new(virtual_address).as_mut_ptr();
        }
    }
    use shos::print;
    // invoke a breakpoint exception
    // You can also panic at any point!
    //panic!("uh oh!");
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
