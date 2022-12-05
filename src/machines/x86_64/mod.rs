core::arch::global_asm!(include_str!("bootloader.s"), options(att_syntax));

mod interrupts;
mod io;
mod vga;

fn halt_loop() -> ! {
	loop {
		unsafe {
			core::arch::asm!("hlt", options(att_syntax));
		}
	}
}

unsafe fn enable_sse() {
	core::arch::asm!(
		"movq %cr0, %rax",
		// "andw ax, 0xFFFB",		// clear coprocessor emulation CR0.EM
		"orq $0x2, %rax",		// set coprocessor monitoring  CR0.MP
		"movq %rax, %cr0",
		"movq %cr4, %rax",
		"orq $(3 << 9), %rax",	// set CR4.OSFXSR and CR4.OSXMMEXCPT at the same time
		"movq %rax, %cr4",

		out("rax") _,
		options(att_syntax)
	);
}

#[no_mangle]
extern "C" fn kmain() -> ! {
	unsafe {
		enable_sse();
	}

	vga::init();
	interrupts::init();

	vga::print_str(b"Hello world");

	halt_loop();
}

#[panic_handler]
fn on_panic(_panic: &core::panic::PanicInfo<'_>) -> ! {
	halt_loop();
}
