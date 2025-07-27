use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};
use crate::println;

// Remember that lazy_statics perform some safe wrapped runtime evaluation
// when the static variable is first initialized. This can cause problems
// if we have multiple lazy_statics, but for this case it makes it possible
// to intialize the idt without a heap
use lazy_static::lazy_static;
use crate::gdt;

lazy_static! {
    // The IDT gets modified to set the handler fn
    // for breakpoints at runtime when its first called.
    // this is okay because IDT won't get modified again after its 
    // initialization
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        // Our double fault handler has to use the good stack index
        // initialized in the gdt
        unsafe {
            idt.double_fault.set_handler_fn(double_fault_handler)
                .set_stack_index(gdt::DOUBLE_FAULT_IST_INDEX);
        }
        idt
    };
}

// initializes an interrupt descriptor table for x86 interrupts
pub fn initialize_idt() {
    IDT.load();
}

extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame) {
    println!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn double_fault_handler(stack_frame: InterruptStackFrame, _error_code: u64) -> ! {
    panic!("EXCEPTION: DOUBLE FAULT\n{:#?}", stack_frame);
}

#[test_case]
fn test_breakpoint_exception() {
    x86_64::instructions::interrupts::int3();
}
