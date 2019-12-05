use libc::{_SC_PAGESIZE, sysconf, posix_memalign, c_void};
use std::{mem, ptr};

pub fn alloc_page() -> *mut c_void {
    let mut memptr: *mut c_void = ptr::null_mut();
    unsafe {
        // FIXME: check whther works
        let size = sysconf(_SC_PAGESIZE) as usize;
        posix_memalign(&mut memptr, size, size);
    }
    memptr
}

pub fn get_page_size() -> usize {
    unsafe { sysconf(_SC_PAGESIZE) as usize }
}
