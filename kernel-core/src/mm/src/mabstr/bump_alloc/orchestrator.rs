use crate::BumpAllocator;

impl BumpAllocator {
    pub const fn new() -> Self {
        BumpAllocator {
            start: 0,
            end: 0,
            next: 0,
        }
    }

    pub unsafe fn init(&mut self, start: usize, size: usize) {
        self.start = start;
        self.end = start + size; // end is the adress where the allocation ends so it is start adress + size
        self.next = start;
    }
}

unsafe impl GlobalAllocator for BumpAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let align = layout.align()
        let size = layout.size()
        let addr = align_up(self.next, align)
        if addr + size > self.end {
            return ptr::null_mut()
        } else {
            (addr as *mut u8).write_bytes(0, size); // Zero initialize
            return addr as *mut u8
        }
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {
        // Nothing yet. Bump Allocator doesn't manages memory disallocation.
    }
}

static ALLOCATOR: BumpAllocator = BumpAllocator::new();
