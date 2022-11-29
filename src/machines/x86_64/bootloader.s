
.global _boot_start

# .extern _part_kernel_begin
# .extern _part_kernel_size_sectors
# .extern kernel_main

.section .boot.16, "ax", %progbits
.code16
_boot_start:
	# Disable interupts
	cli

	# disk number stored in `dl` register
	movb %dl, (boot_disk_number)

	# Read kernel part from disk
	movb $0x02, %ah                     # cmd: read disk
	mov $_part_kernel_size_sectors, %al  # number of sectors to read
	mov $_part_kernel_begin, %bx         # where to store result
	movb $0x00, %ch                     # cylinder (track)
	movb $0x02, %cl                     # sector number
	movb $0x00, %dh                     # head (side)
	movb (boot_disk_number), %dl
	int $0x13

	jc err_16

	# Enablde A20
	inb $0x92, %al
	orb $2, %al
	outb %al, $0x92

# For testing
#	movb $0x0E, %ah
#	movb $64, %al
#	int $0x10
#	int $0x10
#	int $0x10
#	int $0x10
#	jmp .

	# Load GDT (Global Descriptor Table)
	lgdt (gdt_descriptor)

	# Set PE (Protection Enable) bit in CR0 (Control Register 0)
	movl %cr0, %eax
	orl $1, %eax
	movl %eax, %cr0

	jmp $codeseg, $next32

err_16:
	hlt
	jmp err_16

.section .boot.32, "ax", %progbits
.code32
next32:
	# Init stack?
	movw $dataseg, %ax
	movw %ax, %ds
	movw %ax, %ss
	movw %ax, %es
	movw %ax, %fs
	movw %ax, %gs

# For testing
#	movb $64, (0xB8000)
#	movb $64, (0xB8002)
#	movb $64, (0xB8004)
#	movb $64, (0xB8006)
#	jmp .

	# Detect CPUID
	pushfl
	popl %eax
	movl %eax, %ecx
	xorl $(1 << 21), %eax

	pushl %eax
	popfl

	pushfl
	popl %eax

	pushl %ecx
	popfl

	xorl %ecx, %eax
	jz err_32

	# Detect long mode
	movl $0x80000001, %eax
	cpuid
	testl $(1 << 29), %edx
	jz err_32

	# Enable paging
	movl $page_table_entry, %edi
	movl %edi, %cr3
	movl $0x2003, (%edi)
	addl $0x1000, %edi
	movl $0x3003, (%edi)
	addl $0x1000, %edi
	movl $0x4003, (%edi)
	addl $0x1000, %edi

	movl $0x00000003, %ebx
	movl $512, %ecx

0:
	movl %ebx, (%edi)
	addl $0x1000, %ebx
	addl $8, %edi
	loop 0b

	movl %cr4, %eax
	orl $(1 << 5), %eax
	movl %eax, %cr4

	movl $0xC0000080, %ecx
	rdmsr
	orl $(1 << 8), %eax
	wrmsr

	movl %cr0, %eax
	orl $(1 << 31), %eax
	movl %eax, %cr0

	# Edit GDT for 64-bit
	movb $0b10101111, (6 + gdt_codedesc)
	movb $0b10101111, (6 + gdt_datadesc)

	jmp $codeseg, $next64

err_32:
	hlt
	jmp err_32

.section .boot.64, "ax", %progbits
.code64
next64:
	jmp kernel_main

.section .boot.data, "aw", %progbits
	boot_disk_number: .byte 0

	# GDT
gdt_nulldesc:
	.long 0
	.long 0
gdt_codedesc:
	.short 0xFFFF # limit
	.short 0x0000 # base low
	.byte 0x00 # base medium
	.byte 0b10011010 # access: pr, privl(2), s, ex, dc, rw, ac
	.byte 0b11001111 # gr, sz, 0, 0, limit(4)
	.byte 0x00 # base high
gdt_datadesc:
	.short 0xFFFF
	.short 0x0000
	.byte 0x00
	.byte 0b10010010
	.byte 0b11001111
	.byte 0x00
gdt_end:

gdt_descriptor:
gdt_size:
	.short gdt_end - gdt_nulldesc - 1
	.long gdt_nulldesc
	.long 0

.set codeseg, gdt_codedesc - gdt_nulldesc
.set dataseg, gdt_datadesc - gdt_nulldesc

.set page_table_entry, 0x1000
