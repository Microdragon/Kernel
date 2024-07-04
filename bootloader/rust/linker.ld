/* Define the program headers we want so the bootloader gives us the right */
/* MMU permissions */
PHDRS
{
    text    PT_LOAD    FLAGS((1 << 0) | (1 << 2)) ; /* Execute + Read */
    rodata  PT_LOAD    FLAGS((1 << 2)) ;            /* Read only */
    data    PT_LOAD    FLAGS((1 << 1) | (1 << 2)) ; /* Write + Read */
    dynamic PT_DYNAMIC FLAGS((1 << 1) | (1 << 2)) ; /* Write + Read */
}

SECTIONS
{
    /* We wanna be placed in the topmost 2GiB of the address space, for optimisations */
    /* and because that is what the Limine spec mandates. */
    /* Any address in this region will do, but often 0xffffffff80000000 is chosen as */
    /* that is the beginning of the region. */
    . = 0xffffffff80000000;

    .text : {
        *(.text .text.*)
        . = ALIGN(4096);

        __md_link_init_text_start = .;
        *(.init.text .init.text.*)
        . = ALIGN(4096);
        __md_link_init_text_end = .;
    } :text

    /* Move to the next memory page for .rodata */
    . += CONSTANT(MAXPAGESIZE);

    /* The built-in `x86_64-unknown-none` target generates relocatable executables */
    /* by default, so we need to include the relocation information (.dynstr, .dynsym, */
    /* and .rela) for the bootloader too properly load the kernel at runtime. */
    .dynsym : {
        *(.dynsym)
    } :rodata

    .dynstr : {
        *(.dynstr)
    } :rodata

    .rela : {
        *(.rela*)
    } :rodata

    .rodata : {
        *(.rodata .rodata.*)
        . = ALIGN(4096);

        __md_link_init_rodata_start = .;
        *(.init.rodata .init.rodata.*)
        . = ALIGN(4096);
        __md_link_init_rodata_end = .;
    } :rodata

    /* Move to the next memory page for .data */
    . += CONSTANT(MAXPAGESIZE);

    /* The dynamic table is used to find the relocation info (declared above), so it */
    /* must be included both in the :data and :dynamic segments. */
    .dynamic : {
        *(.dynamic)
    } :data :dynamic

    .got : {
        *(.got)
    } :data

    .data : {
        __md_link_init_cell_start = .;
        *(.init.cell .init.cell.*)
        . = ALIGN(4096);
        __md_link_init_cell_end = .;

        *(.data.rel.ro .data.rel.ro.*)
        *(.data .data.*)
        . = ALIGN(4096);

        __md_link_init_data_start = .;
        *(.init.data .init.data.*)
        . = ALIGN(4096);
        __md_link_init_data_end = .;
    } :data

    /* NOTE: .bss needs to be the last thing mapped to :data, otherwise lots of */
    /* unnecessary zeros will be written to the binary. */
    /* If you need, for example, .init_array and .fini_array, those should be placed */
    /* above this. */
    .bss : {
        *(COMMON)
        *(.dynbss)
        *(.bss .bss.*)
    } :data

    /* Discard .note.* and .eh_frame since they may cause issues on some hosts. */
    /DISCARD/ : {
        *(.eh_frame)
        *(.note .note.*)
    }
}