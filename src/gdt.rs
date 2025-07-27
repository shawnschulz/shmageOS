use x86_64::VirtAddr;
use x86_64::structures::tss::TaskStateSegment;
use x86_64::structures::gdt::{GlobalDescriptorTable, Descriptor};
use lazy_static::lazy_static;
use x86_64::structures::gdt::SegmentSelector;


pub const DOUBLE_FAULT_IST_INDEX: u16 = 0;

// Defines the task state segment by creating an interrupt_stack table
// and tss abstraction and adding a stack that can be used for 
// faults where the portions of the stack where exception handlers live
// can't be accessed (uses lazy static for run time mutating on initialization)
lazy_static! {
    static ref TSS: TaskStateSegment = {
        let mut tss = TaskStateSegment::new();
        tss.interrupt_stack_table[DOUBLE_FAULT_IST_INDEX as usize] = {
            const STACK_SIZE: usize = 4096 * 5;
            static mut STACK: [u8; STACK_SIZE] = [0; STACK_SIZE];

            let stack_start = VirtAddr::from_ptr(&raw const STACK);
            let stack_end = stack_start + STACK_SIZE;
            stack_end
        };
        tss
    };
}

struct Selectors {
    code_selector: SegmentSelector,
    tss_selector: SegmentSelector,
}

// Defines the global descriptor table used to load tss (and switch between
// kernel and user space) 
lazy_static! {
    static ref GDT: (GlobalDescriptorTable, Selectors) = {
        let mut gdt = GlobalDescriptorTable::new();
        let code_selector = gdt.add_entry(Descriptor::kernel_code_segment());
        let tss_selector = gdt.add_entry(Descriptor::tss_segment(&TSS));
        (gdt, Selectors { code_selector, tss_selector })
    };
}

pub fn init() {
    use x86_64::instructions::tables::load_tss;
    use x86_64::instructions::segmentation::{CS, Segment};
    GDT.0.load();
    // Loading an invalid selector actually memory leaks, so this is unsafe
    unsafe {
        // To actually properly use the GDT in the case of certain errors
        // (like stack overflow) we must reload the code segment register
        // since the gdt was changed to add entries for the tss_segment,
        // then load the tss, since the GDT uses the TSS selector to actually
        // switch to a good stack
        CS::set_reg(GDT.1.code_selector);
        load_tss(GDT.1.tss_selector);
        // We are not quite done because the CPU needs to actually use the
        // new double fault stack, which we can do by modifying the
        // double fault IDT entry
    }
}
