core::arch::global_asm!(include_str!("bootloader.s"), options(att_syntax));

mod interrupts;
mod io;
mod keyboard;
mod vga;

fn halt_loop() -> ! {
	loop {
		unsafe {
			core::arch::asm!("hlt", options(att_syntax));
		}
	}
}

#[no_mangle]
extern "C" fn kmain() -> ! {
	vga::init();
	interrupts::init();

	vga::print_str(b"Hello world");

	halt_loop();
}

#[panic_handler]
fn on_panic(_panic: &core::panic::PanicInfo<'_>) -> ! {
	halt_loop();
}
