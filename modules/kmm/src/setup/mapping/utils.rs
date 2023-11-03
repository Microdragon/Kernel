use common::addr::VirtAddr;
use core::ptr;
use memory::page_table::{PageTable, PageTableEntry, PageTableFlags};

pub fn read_entry(entry: &mut PageTableEntry, flags: PageTableFlags) -> *mut PageTable {
    if entry.is_unused() {
        let table = crate::pmm::allocate();

        let ptr = VirtAddr::new_truncate(table.start_address().as_u64());
        unsafe { ptr::write_bytes(ptr.as_mut_ptr::<u8>(), 0, 4096) };

        entry.set_addr(table.start_address(), flags);

        ptr.as_mut_ptr::<PageTable>()
    } else {
        VirtAddr::new_truncate(entry.addr().as_u64()).as_mut_ptr::<PageTable>()
    }
}
