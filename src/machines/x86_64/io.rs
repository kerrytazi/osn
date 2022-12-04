pub unsafe fn outb(port: u16, val: u8) {
	core::arch::asm!("outb %al, %dx", in("dx") port, in("al") val, options(att_syntax));
}

pub unsafe fn inb(port: u16) -> u8 {
	let val: u8;
	core::arch::asm!("inb %dx, %al", in("dx") port, out("al") val, options(att_syntax));
	return val;
}
