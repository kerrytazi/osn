ENTRY(_boot_start)

SECTIONS {
	. = 0x7C00;

	.boot : {
		_part_boot_begin = .;

		*(.boot*)

		. = 0x7C00 + 510;

		BYTE(0x55)
		BYTE(0xAA)

		_part_boot_end = .;
	}

	.kernel : {
		_part_kernel_begin = .;

		*(.text* .rodata* .data* .got* .bss*)

		. = ALIGN(512);

		_part_kernel_end = .;
	}

	_part_kernel_size = _part_kernel_end - _part_kernel_begin;
	_part_kernel_size_sectors = _part_kernel_size / 512;
}
