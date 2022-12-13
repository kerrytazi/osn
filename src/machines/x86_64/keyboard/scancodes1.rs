#[allow(dead_code)]
#[derive(Clone, PartialEq, PartialOrd, Ord, Eq, Hash, Copy)]
#[repr(u8)]
pub enum Key {
	Invalid = 0,
	Escape = 1,
	K1 = 2,
	K2 = 3,
	K3 = 4,
	K4 = 5,
	K5 = 6,
	K6 = 7,
	K7 = 8,
	K8 = 9,
	K9 = 10,
	K0 = 11,
	Minus = 12,
	Equal = 13,
	BackSpace = 14,
	Tab = 15,
	Q = 16,
	W = 17,
	E = 18,
	R = 19,
	T = 20,
	Y = 21,
	U = 22,
	I = 23,
	O = 24,
	P = 25,
	BracketOpen = 26,
	BracketClose = 27,
	Enter = 28,
	LeftCtrl = 29,
	A = 30,
	S = 31,
	D = 32,
	F = 33,
	G = 34,
	H = 35,
	J = 36,
	K = 37,
	L = 38,
	Semicolon = 39,
	SingleQuote = 40,
	BackTick = 41,
	LeftShift = 42,
	BackSlash = 43,
	Z = 44,
	X = 45,
	C = 46,
	V = 47,
	B = 48,
	N = 49,
	M = 50,
	Comma = 51,
	Dot = 52,
	Slash = 53,
	RightShift = 54,
	KpMultiply = 55,
	LeftAlt = 56,
	Space = 57,
	CapsLock = 58,
	F1 = 59,
	F2 = 60,
	F3 = 61,
	F4 = 62,
	F5 = 63,
	F6 = 64,
	F7 = 65,
	F8 = 66,
	F9 = 67,
	F10 = 68,
	NumberLock = 69,
	ScrollLock = 70,
	Kp7 = 71,
	Kp8 = 72,
	Kp9 = 73,
	KpMinus = 74,
	Kp4 = 75,
	Kp5 = 76,
	Kp6 = 77,
	KpPlus = 78,
	Kp1 = 79,
	Kp2 = 80,
	Kp3 = 81,
	Kp0 = 82,
	KpDot = 83,
	F11 = 84,
	F12 = 85,

	_MaxValue = 86,
}

pub struct ScanToKeyResult {
	pub key: Key,
	pub pressed: bool,
}

pub const fn scan_to_key(scan: u8) -> ScanToKeyResult {
	let pressed = (scan & 0b_1000_0000) == 0;
	let scan = scan & 0b_0111_1111;

	let key = if scan > 85 {
		Key::Invalid
	} else {
		unsafe { core::mem::transmute(scan) }
	};

	ScanToKeyResult {
		key: key,
		pressed: pressed,
	}
}

const fn create_key_to_ascii_table(shift: bool) -> [u8; Key::_MaxValue as usize] {
	let mut table = [0; Key::_MaxValue as usize];

	if shift {
		table[Key::BackTick as usize] = b'~';

		table[Key::K1 as usize] = b'!';
		table[Key::K2 as usize] = b'@';
		table[Key::K3 as usize] = b'#';
		table[Key::K4 as usize] = b'$';
		table[Key::K5 as usize] = b'%';
		table[Key::K6 as usize] = b'^';
		table[Key::K7 as usize] = b'&';
		table[Key::K8 as usize] = b'*';
		table[Key::K9 as usize] = b'(';
		table[Key::K0 as usize] = b')';

		table[Key::Q as usize] = b'Q';
		table[Key::W as usize] = b'W';
		table[Key::E as usize] = b'E';
		table[Key::R as usize] = b'R';
		table[Key::T as usize] = b'T';
		table[Key::Y as usize] = b'Y';
		table[Key::U as usize] = b'U';
		table[Key::I as usize] = b'I';
		table[Key::O as usize] = b'O';
		table[Key::P as usize] = b'P';
		table[Key::BracketOpen as usize] = b'{';
		table[Key::BracketClose as usize] = b'}';
		table[Key::BackSlash as usize] = b'|';

		table[Key::A as usize] = b'A';
		table[Key::S as usize] = b'S';
		table[Key::D as usize] = b'D';
		table[Key::F as usize] = b'F';
		table[Key::G as usize] = b'G';
		table[Key::H as usize] = b'H';
		table[Key::J as usize] = b'J';
		table[Key::K as usize] = b'K';
		table[Key::L as usize] = b'L';
		table[Key::Semicolon as usize] = b':';
		table[Key::SingleQuote as usize] = b'"';

		table[Key::Z as usize] = b'Z';
		table[Key::X as usize] = b'X';
		table[Key::C as usize] = b'C';
		table[Key::V as usize] = b'V';
		table[Key::B as usize] = b'B';
		table[Key::N as usize] = b'N';
		table[Key::M as usize] = b'M';
		table[Key::Comma as usize] = b'<';
		table[Key::Dot as usize] = b'>';
		table[Key::Slash as usize] = b'?';
	} else {
		table[Key::BackTick as usize] = b'`';

		table[Key::K1 as usize] = b'1';
		table[Key::K2 as usize] = b'2';
		table[Key::K3 as usize] = b'3';
		table[Key::K4 as usize] = b'4';
		table[Key::K5 as usize] = b'5';
		table[Key::K6 as usize] = b'6';
		table[Key::K7 as usize] = b'7';
		table[Key::K8 as usize] = b'8';
		table[Key::K9 as usize] = b'9';
		table[Key::K0 as usize] = b'0';

		table[Key::Q as usize] = b'q';
		table[Key::W as usize] = b'w';
		table[Key::E as usize] = b'e';
		table[Key::R as usize] = b'r';
		table[Key::T as usize] = b't';
		table[Key::Y as usize] = b'y';
		table[Key::U as usize] = b'u';
		table[Key::I as usize] = b'i';
		table[Key::O as usize] = b'o';
		table[Key::P as usize] = b'p';
		table[Key::BracketOpen as usize] = b'[';
		table[Key::BracketClose as usize] = b']';
		table[Key::BackSlash as usize] = b'\\';

		table[Key::A as usize] = b'a';
		table[Key::S as usize] = b's';
		table[Key::D as usize] = b'd';
		table[Key::F as usize] = b'f';
		table[Key::G as usize] = b'g';
		table[Key::H as usize] = b'h';
		table[Key::J as usize] = b'j';
		table[Key::K as usize] = b'k';
		table[Key::L as usize] = b'l';
		table[Key::Semicolon as usize] = b';';
		table[Key::SingleQuote as usize] = b'\'';

		table[Key::Z as usize] = b'z';
		table[Key::X as usize] = b'x';
		table[Key::C as usize] = b'c';
		table[Key::V as usize] = b'v';
		table[Key::B as usize] = b'b';
		table[Key::N as usize] = b'n';
		table[Key::M as usize] = b'm';
		table[Key::Comma as usize] = b',';
		table[Key::Dot as usize] = b'.';
		table[Key::Slash as usize] = b'/';
	}

	table[Key::Kp1 as usize] = b'1';
	table[Key::Kp2 as usize] = b'2';
	table[Key::Kp3 as usize] = b'3';
	table[Key::Kp4 as usize] = b'4';
	table[Key::Kp5 as usize] = b'5';
	table[Key::Kp6 as usize] = b'6';
	table[Key::Kp7 as usize] = b'7';
	table[Key::Kp8 as usize] = b'8';
	table[Key::Kp9 as usize] = b'9';
	table[Key::Kp0 as usize] = b'0';

	table[Key::KpMultiply as usize] = b'*';
	table[Key::KpMinus as usize] = b'-';
	table[Key::KpPlus as usize] = b'+';
	table[Key::KpDot as usize] = b'.';

	table
}

const KEY_TO_ASCII_TABLE: [u8; Key::_MaxValue as usize] = create_key_to_ascii_table(false);
const KEY_TO_ASCII_SHIFT_TABLE: [u8; Key::_MaxValue as usize] = create_key_to_ascii_table(true);

pub const fn key_to_ascii(key: Key, shift: bool) -> u8 {
	let key_u8 = unsafe { core::mem::transmute::<Key, u8>(key) };

	assert!(key_u8 < Key::_MaxValue as u8);

	if shift {
		KEY_TO_ASCII_SHIFT_TABLE[key_u8 as usize]
	} else {
		KEY_TO_ASCII_TABLE[key_u8 as usize]
	}
}
