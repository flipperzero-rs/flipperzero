ENTRY(_start);

SECTIONS
{
    .text 0x00000000 : ALIGN(4)
    {
        KEEP(*(.init));
        *(.text .text.*);
        KEEP(*(.fini));
    }

    .rodata :
    {
        *(.rodata .rodata.*);
    }

    .bss :
    {
        *(.bss .bss.*);
    }

    .data :
    {
        *(.data .data.*);
    }

	.ARM.attributes :
	{
		*(.ARM.attributes .ARM.attributes.*);
	}

    .fapmeta (INFO) :
    {
        KEEP(*(.fapmeta));
    }

    /DISCARD/ :
    {
        *(.text.strlen);

        /* LLVM bitcode should never be in the binary */
        *(.llvmbc .llvmcmd);

        /* Unwinding is not supported */
        *(.ARM.exidx .ARM.exidx.*);

        *(.comment .comment.*);
        *(.note .note.*);
        *(.debug*);
    }
}
