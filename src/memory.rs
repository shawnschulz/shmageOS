use x86_64::{ 
    structures::paging::PageTable, PhysAddr, VirtAddr
};

// Caller responsibility to guarantee that physical memory is mapped to
// virtual memory at the VirtAddr offset 'physical_memory_offset'. Kernel
// should only call this function once (no aliasing &mut references)
pub unsafe fn active_level_4_table(physical_memory_offset: VirtAddr) -> &'static mut PageTable { 
    // Reminder that Cr3 contains the address of the level 4 page table (highest level)
    use x86_64::registers::control::Cr3;
    let (level_4_table_frame, _) = Cr3::read();
    let physical_address = level_4_table_frame.start_address();
    let virtual_address = physical_memory_offset + physical_address.as_u64();
    let page_table_ptr: *mut PageTable = virtual_address.as_mut_ptr();

    unsafe { &mut *page_table_ptr }
}

// Translates the given virtual address into the mapped physical address, or None if there is no
// mapping
// Unsafe, caller must guarantee that complete physical memory is mapped at the physical memory
// offset
// (since the entire function is unsafe, want to wrap around a safe function with specific unsafe
// blocks
pub unsafe fn translate_address(address: VirtAddr, physical_memory_offset: VirtAddr) -> Option<PhysAddr> {
    translate_address_inner(address, physical_memory_offset)
}

pub fn translate_address_inner(address: VirtAddr, physical_memory_offset: VirtAddr) -> Option<PhysAddr> {
    use x86_64::structures::paging::page_table::FrameError;
    use x86_64::registers::control::Cr3;
    // grab our level 4 page table
    let (level_4_page_table_frame, _) = Cr3::read();
    let table_indices = [
        address.p4_index(), address.p3_index(), address.p2_index(), address.p1_index()
    ];
    let mut frame = level_4_page_table_frame;
    // Traverse the multi level page table
    for &index in &table_indices {
        let virtual_address = physical_memory_offset + frame.start_address().as_u64();
        let table_ptr: *const PageTable = virtual_address.as_ptr();
        // deref the page table pointer
        let table = unsafe {&*table_ptr};
        // read the entry
        let entry = &table[index];
        // mutate the frame
        frame = match entry.frame() {
            Ok(frame) => frame,
            Err(FrameError::FrameNotPresent) => return None,
            Err(FrameError::HugeFrame) => panic!("Huge frames not supported"),
        };
    };
    Some(frame.start_address() + u64::from(address.page_offset()))
}
