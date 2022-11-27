
[global _boot_start]

[extern _part_kernel_begin]
[extern _part_kernel_size_sectors]
[extern kernel_main]


[section .boot.16]
[bits 16]
_boot_start:

; Code
	mov byte [G_boot_disk_number], dl ; disk number stored in `dl` register

	; Read part 2 from disk
	mov ah, 0x02 ; cmd: read disk
	mov al, _part_kernel_size_sectors ; number of sectors to read
	mov bx, _part_kernel_begin ; where to store result
	mov ch, 0x00 ; cylinder (track)
	mov cl, 0x02 ; sector number
	mov dh, 0x00 ; head (side)
	mov dl, [G_boot_disk_number]
	int 0x13

	jc L_b1_disk_err
	jmp L_after_disk_read

L_b1_disk_err:
	mov bp, D_str_disk_read_err
	mov cx, D_str_disk_read_err_end
	call P_16_print

	jmp $ ; halt

P_16_print:
	mov ah, 0x0E
	mov al, byte [bp]
	int 0x10

	inc bp
	cmp bp, cx
	jnz P_16_print

	ret

L_after_disk_read:
	mov bp, D_str_success
	mov cx, D_str_success_end
	call P_16_print

	; Enter protected mode

	; Enablde A20
	in al, 0x92
	or al, 2
	out 0x92, al

	; Disable interupts
	cli

	; Load GDT (Global Descriptor Table)
	lgdt [D_gdt_descriptor]

	; Set PE (Protection Enable) bit in CR0 (Control Register 0)
	mov eax, cr0
	or eax, 1
	mov cr0, eax

	jmp D_codeseg:L_enter_protected_mode

[section .boot.32]
[bits 32]
L_enter_protected_mode:
	; Init stack?
	mov ax, D_dataseg
	mov ds, ax
	mov ss, ax
	mov es, ax
	mov fs, ax
	mov gs, ax

	; Test video mode
	mov ebp, D_str_entered32
	mov ecx, 0
L_te
	mov al, byte [ebp+ecx]
	mov byte [0xB8000+ecx*2], al
	inc ecx
	cmp ecx, D_str_entered32_len
	jnz L_te

	; Detect CPUID
	pushfd
	pop eax
	mov ecx, eax
	xor eax, 1 << 21

	push eax
	popfd

	pushfd
	pop eax

	push ecx
	popfd

	xor eax, ecx
	jz L_32_halt

	; Detect long mode
	mov eax, 0x80000001
	cpuid
	test edx, 1 << 29
	jz L_32_halt

	; Enable paging
	mov edi, D_page_table_entry
	mov cr3, edi
	mov dword [edi], 0x2003
	add edi, 0x1000
	mov dword [edi], 0x3003
	add edi, 0x1000
	mov dword [edi], 0x4003
	add edi, 0x1000

	mov ebx, 0x00000003
	mov ecx, 512

L_paging_set_entry:
	mov dword [edi], ebx
	add ebx, 0x1000
	add edi, 8
	loop L_paging_set_entry

	mov eax, cr4
	or eax, 1 << 5
	mov cr4, eax

	mov ecx, 0xC0000080
	rdmsr
	or eax, 1 << 8
	wrmsr

	mov eax, cr0
	or eax, 1 << 31
	mov cr0, eax

	; Edit GDT for 64-bit
	mov byte [D_gdt_codedesc + 6], 10101111_b
	mov byte [D_gdt_datadesc + 6], 10101111_b

	jmp D_codeseg:L_enter_64_mode

L_32_halt:
	hlt

[section .boot.64]
[bits 64]
L_enter_64_mode:
	mov edi, 0xB8000
	mov rax, 0x1f201f201f201f20
	mov ecx, 500
	rep stosq

	jmp kernel_main

	jmp $ ; halt

; Data
	D_page_table_entry equ 0x1000

	; GDT
D_gdt_nulldesc:
	dw 0x0000
	dw 0x0000
	db 0x00
	db 0x00
	db 0x00
	db 0x00
D_gdt_codedesc:
	dw 0xFFFF ; limit
	dw 0x0000 ; base low
	db 0x00 ; base medium
	db 10011010_b ; access: pr, privl(2), s, ex, dc, rw, ac
	db 11001111_b ; gr, sz, 0, 0, limit(4)
	db 0x00 ; base high
D_gdt_datadesc:
	dw 0xFFFF
	dw 0x0000
	db 0x00
	db 10010010_b
	db 11001111_b
	db 0x00
D_gdt_end:

D_gdt_descriptor:
D_gdt_size:
	dw D_gdt_end - D_gdt_nulldesc - 1
	dq D_gdt_nulldesc

D_codeseg equ D_gdt_codedesc - D_gdt_nulldesc
D_dataseg equ D_gdt_datadesc - D_gdt_nulldesc

	D_str_success db "Loaded successfully",0xD,0xA
	D_str_success_end equ $

	D_str_entered32 db "Entered 32-bit mode"
	D_str_entered32_len equ $

	D_str_disk_read_err db "Disk read failed",0xD,0xA
	D_str_disk_read_err_end equ $

	G_boot_disk_number db 0
