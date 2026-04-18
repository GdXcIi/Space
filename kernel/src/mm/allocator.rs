use core::alloc::GlobalAlloc;
use core::ptr::null_mut;

struct BumpAllocator {
    heap_start: usize,
    heap_end: usize,
    next: usize,
}

/// SAFETY: We need to use `unsafe` because we need to use pointers to implement GlobalAlloc.
unsafe impl GlobalAlloc for BumpAlloc {
    unsafe fn alloc(&sef, layout: core::alloc::Layout) -> *mut u8 {
        let alloc_start = self.next;
        let alloc_end = alloc_start + layout.size()

        if alloc_end > self.heap_end {
            null_mut()
        } else {
            self.next = alloc_end;
            alloc_start as *mut u8
        }
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: core::alloc::Layout) {
        // nothing (bump allocator = no free)
    }
}
