core::arch::global_asm!(include_str!("bootloader.s"), options(att_syntax));

#[no_mangle]
pub extern "C" fn kernel_main() -> ! {
	static HELLO: &[u8] = b"Hello World!";

	let vga_buffer = 0xB8000 as *mut u8;

	for (i, &byte) in HELLO.iter().enumerate() {
		unsafe {
			*vga_buffer.offset(i as isize * 2) = byte;
			*vga_buffer.offset(i as isize * 2 + 1) = 0xb;
		}
	}

	loop {}
}

#[panic_handler]
fn on_panic(_panic: &core::panic::PanicInfo<'_>) -> ! {
	loop {}
}
