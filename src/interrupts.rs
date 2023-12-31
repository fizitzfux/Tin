use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame, PageFaultErrorCode};
use crate::println;
use lazy_static::lazy_static;

use crate::serial_println;

// pub fn init_idt() {
//     let mut idt = InterruptDescriptorTable::new();
//     idt.breakpoint.set_handler_fn(breakpoint_handler);
// }

extern "x86-interrupt" fn breakpoint_handler(
    stack_frame: InterruptStackFrame)
{
    println!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
}
extern "x86-interrupt" fn page_fault_handler(
    stack_frame: InterruptStackFrame, error_code: PageFaultErrorCode)
{
    panic!("EXCEPTION {:08b}: PAGE FAULT\n{:#?}", error_code, stack_frame);
}
extern "x86-interrupt" fn double_fault_handler(
    stack_frame: InterruptStackFrame, _error_code: u64) -> !
{
    panic!("EXCEPTION: DOUBLE FAULT\n{:#?}", stack_frame);
}

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        idt.page_fault.set_handler_fn(page_fault_handler);
        idt.double_fault.set_handler_fn(double_fault_handler);
        idt
    };
}

pub fn init_idt() {
    IDT.load();
}

#[test_case]
fn test_breakpoint_exception() {
    // invoke a breakpoint exception
    x86_64::instructions::interrupts::int3();
}