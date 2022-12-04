core::arch::global_asm!(include_str!("bootloader.s"), options(att_syntax));

mod io;
mod vga;

#[no_mangle]
pub extern "C" fn kernel_main() -> ! {
	vga::init();

	vga::print_str(b"Hello world");

	loop {}
}

#[panic_handler]
fn on_panic(_panic: &core::panic::PanicInfo<'_>) -> ! {
	loop {}
}
