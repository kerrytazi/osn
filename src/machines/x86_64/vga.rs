pub mod consts {
	pub const SCREEN_WIDTH: u16 = 80;
	pub const SCREEN_HEIGHT: u16 = 25;

	pub const SCREEN_MAX_POS: u16 = SCREEN_WIDTH * SCREEN_HEIGHT;
}

static mut CURRENT_VGA_POS: u16 = 0;

const VGA_BUFFER: *mut u8 = 0xB8000 as *mut u8;

fn get_screen_buffer() -> &'static mut [[u8; 2]] {
	unsafe {
		(&mut *core::ptr::slice_from_raw_parts_mut(VGA_BUFFER, 2 * consts::SCREEN_MAX_POS as usize))
			.as_chunks_unchecked_mut::<2>()
	}
}

#[allow(dead_code)]
pub mod color {
	pub mod foreground {
		pub const BLACK: u8 = 0x00;
		pub const BLUE: u8 = 0x01;
		pub const GREEN: u8 = 0x02;
		pub const CYAN: u8 = 0x03;
		pub const RED: u8 = 0x04;
		pub const PURPLE: u8 = 0x05;
		pub const BROWN: u8 = 0x06;
		pub const GRAY: u8 = 0x07;
		pub const DARK_GRAY: u8 = 0x08;
		pub const LIGHT_BLUE: u8 = 0x09;
		pub const LIGHT_GREEN: u8 = 0x0A;
		pub const LIGHT_CYAN: u8 = 0x0B;
		pub const LIGHT_RED: u8 = 0x0C;
		pub const LIGHT_PURPLE: u8 = 0x0D;
		pub const YELLOW: u8 = 0x0E;
		pub const WHITE: u8 = 0x0F;
	}

	pub mod background {
		pub const BLACK: u8 = 0x00;
		pub const BLUE: u8 = 0x10;
		pub const GREEN: u8 = 0x20;
		pub const CYAN: u8 = 0x30;
		pub const RED: u8 = 0x40;
		pub const PURPLE: u8 = 0x50;
		pub const BROWN: u8 = 0x60;
		pub const GRAY: u8 = 0x70;

		pub const DARK_GRAY: u8 = 0x80;
		pub const LIGHT_BLUE: u8 = 0x90;
		pub const LIGHT_GREEN: u8 = 0xA0;
		pub const LIGHT_CYAN: u8 = 0xB0;
		pub const LIGHT_RED: u8 = 0xC0;
		pub const LIGHT_PURPLE: u8 = 0xD0;
		pub const YELLOW: u8 = 0xE0;
		pub const WHITE: u8 = 0xF0;

		pub const BLINKING_BLACK: u8 = 0x80;
		pub const BLINKING_BLUE: u8 = 0x90;
		pub const BLINKING_GREEN: u8 = 0xA0;
		pub const BLINKING_CYAN: u8 = 0xB0;
		pub const BLINKING_RED: u8 = 0xC0;
		pub const BLINKING_PURPLE: u8 = 0xD0;
		pub const BLINKING_YELLOW: u8 = 0xE0;
		pub const BLINKING_WHITE: u8 = 0xF0;
	}

	pub const BLACK: u8 = foreground::BLACK | background::BLACK;
	pub const DEFAULT: u8 = foreground::WHITE | background::BLACK;
}

pub fn init() {
	#[cfg(debug_assertions)]
	unsafe {
		static mut WAS: bool = false;
		assert!(!WAS);
		WAS = true;
	}

	clear_screen();
	set_cursor_pos(0);
}

pub fn set_cursor_pos(pos: u16) {
	assert!(pos < consts::SCREEN_MAX_POS);

	let pos_bytes = pos.to_ne_bytes();

	unsafe {
		super::io::outb(0x3D4, 0x0F);
		super::io::outb(0x3D5, pos_bytes[0]);
		super::io::outb(0x3D4, 0x0E);
		super::io::outb(0x3D5, pos_bytes[1]);
	}
}

pub fn coord_to_pos(x: u16, y: u16) -> u16 {
	assert!(x < consts::SCREEN_WIDTH);
	assert!(y < consts::SCREEN_HEIGHT);

	return x + y * consts::SCREEN_WIDTH;
}

pub fn print_str(s: &[u8]) {
	print_str_colored(s, color::DEFAULT);
}

pub fn print_str_colored(s: &[u8], color: u8) {
	let vga_buffer = get_screen_buffer();
	let prev = unsafe { CURRENT_VGA_POS as usize };

	for (i, &ch) in s.iter().enumerate() {
		vga_buffer[prev + i][0] = ch;
		vga_buffer[prev + i][1] = color;
	}

	set_cursor_pos((prev + s.len()) as u16);
}

pub fn clear_screen() {
	clear_screen_colored(0x00, color::BLACK);
}

pub fn clear_screen_colored(ch: u8, color: u8) {
	let vga_buffer = get_screen_buffer();

	for pair in vga_buffer {
		pair[0] = ch;
		pair[1] = color;
	}
}
