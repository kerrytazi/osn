use super::io::port::{inb, io_wait, outb};

#[allow(dead_code)]
mod consts {
	pub const PIC1: u16 = 0x20; // IO base address for master PIC
	pub const PIC2: u16 = 0xA0; // IO base address for slave PIC
	pub const PIC1_COMMAND: u16 = PIC1;
	pub const PIC1_DATA: u16 = PIC1 + 1;
	pub const PIC2_COMMAND: u16 = PIC2;
	pub const PIC2_DATA: u16 = PIC2 + 1;

	pub const PIC_EOI: u8 = 0x20;

	pub const ICW1_ICW4: u8 = 0x01; // ICW4 (not) needed
	pub const ICW1_SINGLE: u8 = 0x02; // Single (cascade) mode
	pub const ICW1_INTERVAL4: u8 = 0x04; // Call address interval 4 (8)
	pub const ICW1_LEVEL: u8 = 0x08; // Level triggered (edge) mode
	pub const ICW1_INIT: u8 = 0x10; // Initialization - required!

	pub const ICW4_8086: u8 = 0x01; // 8086/88 (MCS-80/85) mode
	pub const ICW4_AUTO: u8 = 0x02; // Auto (normal) EOI
	pub const ICW4_BUF_SLAVE: u8 = 0x08; // Buffered mode/slave
	pub const ICW4_BUF_MASTER: u8 = 0x0C; // Buffered mode/master
	pub const ICW4_SFNM: u8 = 0x10; // Special fully nested (not)
}

unsafe fn end_of_interrupt(irq: u8) {
	if irq >= 8 {
		outb(consts::PIC2_COMMAND, consts::PIC_EOI);
	}

	outb(consts::PIC1_COMMAND, consts::PIC_EOI);
}

unsafe fn remap_pic(offset1: u8, offset2: u8) {
	let prev_pic1_data = inb(consts::PIC1_DATA); // save masks
	let prev_pic2_data = inb(consts::PIC2_DATA);

	outb(consts::PIC1_COMMAND, consts::ICW1_INIT | consts::ICW1_ICW4); // starts the initialization sequence (in cascade mode)
	io_wait();
	outb(consts::PIC2_COMMAND, consts::ICW1_INIT | consts::ICW1_ICW4);
	io_wait();
	outb(consts::PIC1_DATA, offset1); // ICW2: Master PIC vector offset
	io_wait();
	outb(consts::PIC2_DATA, offset2); // ICW2: Slave PIC vector offset
	io_wait();
	outb(consts::PIC1_DATA, 4); // ICW3: tell Master PIC that there is a slave PIC at IRQ2 (0000 0100)
	io_wait();
	outb(consts::PIC2_DATA, 2); // ICW3: tell Slave PIC its cascade identity (0000 0010)
	io_wait();

	outb(consts::PIC1_DATA, consts::ICW4_8086);
	io_wait();
	outb(consts::PIC2_DATA, consts::ICW4_8086);
	io_wait();

	outb(consts::PIC1_DATA, prev_pic1_data); // restore saved masks.
	outb(consts::PIC2_DATA, prev_pic2_data);
}

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
unsafe extern "x86-interrupt" fn kisr1() {
	use super::keyboard::scancodes1 as sc1;

	let scan = inb(0x60);

	let sc1::ScanToKeyResult { key, pressed } = sc1::scan_to_key(scan);

	super::vga::print_str(b"kisr1: ");

	if key == sc1::Key::Invalid {
		super::vga::print_str(b"invalid");
	} else {
		if pressed {
			super::vga::print_str(b"pressed: ");
		} else {
			super::vga::print_str(b"unpressed: ");
		}

		let shift_pressed = false; // TODO

		super::vga::print_str(&[sc1::key_to_ascii(key, shift_pressed)]);
	}

	super::vga::print_str(b"   ");

	end_of_interrupt(1);
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
		INTERRUPT_DESCRIPTOR_TABLE[1].init(
			kisr1,
			attribute_type::gate::INTERRUPT | attribute_type::PRESENT_BIT,
		);

		remap_pic(0, 8);

		outb(0x21, 0xFD);
		outb(0xA1, 0xFF);

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
