#[repr(C, packed)]
#[derive(Copy, Clone, Eq, PartialEq)]
struct InterruptDescriptor {
	isr_low: u16,
	cs: u16,
	ist: u8,
	attributes: u8,
	isr_mid: u16,
	isr_high: u32,
	rsv0: u32,
}

impl InterruptDescriptor {
	const fn new() -> Self {
		InterruptDescriptor {
			isr_low: 0,
			cs: 0,
			ist: 0,
			attributes: 0,
			isr_mid: 0,
			isr_high: 0,
			rsv0: 0,
		}
	}

	fn init(&mut self, isr: unsafe extern "x86-interrupt" fn(), attributes: u8) {
		let isr = isr as usize;

		self.isr_low = (isr & 0x_0000_0000_0000_FFFF_usize) as u16;
		self.cs = 0x08; // TODO: GDT_OFFSET_KERNEL_CODE
		self.ist = 0; // TODO: Interrupt Stack Table https://wiki.osdev.org/Task_State_Segment
		self.attributes = attributes; // TODO: bitflags
		self.isr_mid = ((isr & 0x_0000_0000_FFFF_0000_usize) >> 16) as u16;
		self.isr_high = ((isr & 0x_FFFF_FFFF_0000_0000_usize) >> 32) as u32;
		self.rsv0 = 0;
	}
}

#[repr(C, packed)]
struct InterruptDescriptorTableDescriptor {
	limit: u16,
	addr: *const InterruptDescriptor,
}

unsafe impl Send for InterruptDescriptorTableDescriptor {}
unsafe impl Sync for InterruptDescriptorTableDescriptor {}

static mut INTERRUPT_DESCRIPTOR_TABLE: [InterruptDescriptor; 256] =
	[InterruptDescriptor::new(); 256];

#[no_mangle]
static g_IDTDescriptor: InterruptDescriptorTableDescriptor = InterruptDescriptorTableDescriptor {
	limit: (core::mem::size_of::<[InterruptDescriptor; 256]>() - 1) as u16,
	addr: unsafe { INTERRUPT_DESCRIPTOR_TABLE.as_ptr() },
};

// #[target_feature(enable = "sse")]
#[no_mangle]
extern "x86-interrupt" fn kisr1() {
	let code = unsafe { super::io::port::inb(0x60) };

	super::vga::print_str(b"kisr1: ");
	super::vga::print_str(&[code]);

	unsafe {
		super::io::port::outb(0x20, 0x20);
		super::io::port::outb(0xA0, 0x20);
	}
}

#[allow(dead_code)]
mod attribute_type {
	/// A 4-bit value which defines the type of gate this Interrupt Descriptor represents
	pub mod gate {
		/// interrupt gates will disable further processor handling of maskable hardware interrupts, making them suitable to handle hardware-generated interrupts
		pub const INTERRUPT: u8 = 0b_0000_1110;
		/// trap gates are useful for handling software interrupts and exceptions
		pub const TRAP: u8 = 0b_0000_1111;
	}

	/// A 2-bit value which defines the CPU Privilege Levels which are allowed to access this interrupt via the INT instruction. Hardware interrupts ignore this mechanism.
	pub mod dpl {
		pub const PRIVILEGE_0: u8 = 0b_0000_0000;
		pub const PRIVILEGE_1: u8 = 0b_0010_0000;
		pub const PRIVILEGE_2: u8 = 0b_0100_0000;
		pub const PRIVILEGE_3: u8 = 0b_0110_0000;
	}

	/// Present bit. Must be set (1) for the descriptor to be valid.
	pub const PRESENT_BIT: u8 = 0b_1000_0000;
}

pub fn init() {
	unsafe {
		for descriptor in INTERRUPT_DESCRIPTOR_TABLE.iter_mut() {
			descriptor.init(
				kisr1,
				attribute_type::gate::INTERRUPT | attribute_type::PRESENT_BIT,
			);
		}

		super::io::port::outb(0x21, 0xFD);
		super::io::port::outb(0xA1, 0xFF);

		load_idt(&g_IDTDescriptor);
		enable();
	}
}

unsafe fn load_idt(idtd: *const InterruptDescriptorTableDescriptor) {
	core::arch::asm!("lidtq ({0})", in(reg) idtd, options(att_syntax));
}

pub unsafe fn disable() {
	core::arch::asm!("cli", options(att_syntax));
}

pub unsafe fn enable() {
	core::arch::asm!("sti", options(att_syntax));
}
