
use lazy_static::lazy_static;
use spin::Mutex;
use x86_64::structures::idt::InterruptDescriptorTable;

lazy_static! {
    pub static ref interrupt_table: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();

        idt.divide_error.set_handler_fn(exceptions::handle_div_err);

        idt
    };
}

pub fn init_interrupt_table() {
    interrupt_table.load();
}

pub mod exceptions {
    use x86_64::structures::idt::InterruptStackFrame;

    pub extern "x86-interrupt" fn handle_div_err(stack_fr: InterruptStackFrame) {
        panic!("[DIVISION ERROR]\n{stack_fr:?}")
    }
}