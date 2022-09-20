use spin::Lazy;
use x86_64::{
    instructions::tables::load_tss,
    registers::segmentation::{Segment, CS, SS},
    structures::{
        gdt::{Descriptor, GlobalDescriptorTable, SegmentSelector},
        idt::{InterruptDescriptorTable, InterruptStackFrame},
        tss::TaskStateSegment,
    },
};

pub const DOUBLE_FAULT_IST: u16 = 0;

static IDT: Lazy<InterruptDescriptorTable> = Lazy::new(|| {
    let mut idt = InterruptDescriptorTable::new();
    idt.breakpoint.set_handler_fn(breakpoint_handler);
    idt.double_fault.set_handler_fn(double_fault_handler);
    unsafe {
        idt.double_fault
            .set_handler_fn(double_fault_handler)
            .set_stack_index(DOUBLE_FAULT_IST);
    }
    idt
});
static TSS: Lazy<TaskStateSegment> = Lazy::new(|| {
    let mut tss = TaskStateSegment::new();
    tss.interrupt_stack_table[DOUBLE_FAULT_IST as usize] = {
        const STACK_SIZE: usize = 5 * 1024; // 5 KiB - minimum stack size to print the panic message
        static mut STACK: [u8; STACK_SIZE] = [0; STACK_SIZE];

        let stack_start = x86_64::VirtAddr::from_ptr(unsafe { &STACK });
        let stack_end = stack_start + STACK_SIZE;
        stack_end
    };
    tss
});
static GDT: Lazy<(GlobalDescriptorTable, (SegmentSelector, SegmentSelector))> = Lazy::new(|| {
    let mut gdt = GlobalDescriptorTable::new();
    let code_selector = gdt.add_entry(Descriptor::kernel_code_segment());
    let tss_selector = gdt.add_entry(Descriptor::tss_segment(&TSS));
    (gdt, (code_selector, tss_selector))
});

pub fn init_idt() {
    GDT.0.load();
    unsafe {
        CS::set_reg(GDT.1 .0);
        SS::set_reg(SegmentSelector(0));
        load_tss(GDT.1 .1);
    }
    IDT.load();
}

extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame) {
    log::warn!("BREAKPOINT\n{stack_frame:#?}");
}

extern "x86-interrupt" fn double_fault_handler(
    stack_frame: InterruptStackFrame,
    _error_code: u64,
) -> ! {
    panic!("DOUBLE FAULT\n{stack_frame:#?}");
}
