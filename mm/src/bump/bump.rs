// TODO: Document this file

use core::{alloc::GlobalAlloc, ptr};

pub struct BumpAllocator {
    pub next: core::cell::UnsafeCell<u64>,
    heap_start: core::cell::UnsafeCell<u64>,
    heap_size: core::cell::UnsafeCell<u64>,
}

unsafe impl Sync for BumpAllocator {}

impl BumpAllocator {
    pub const fn new() -> Self {
        BumpAllocator {
            next: core::cell::UnsafeCell::new(0),
            heap_start: core::cell::UnsafeCell::new(0),
            heap_size: core::cell::UnsafeCell::new(0),
        }
    }

    pub fn init(&self, boot_i: &boot::BootInfo) {
        unsafe {
            *self.next.get() = boot_i.heap_start;
            *self.heap_start.get() = boot_i.heap_start;
            *self.heap_size.get() = boot_i.heap_end - boot_i.heap_start;
        }
    }
}

unsafe impl GlobalAlloc for BumpAllocator {
    unsafe fn alloc(&self, layout: core::alloc::Layout) -> *mut u8 {
        let heap_start = unsafe { *self.heap_start.get() };
        let heap_size = unsafe { *self.heap_size.get() };
        let next_ptr = self.next.get();

        if heap_size == 0 {
            return ptr::null_mut();
        }

        let current_next = unsafe { *next_ptr };

        let alloc_start = current_next.next_multiple_of(layout.align() as u64);

        let alloc_end = match alloc_start.checked_add(layout.size() as u64) {
            Some(end) => end,
            None => return ptr::null_mut(),
        };

        if alloc_end <= heap_start + heap_size {
            unsafe { *next_ptr = alloc_end };
            alloc_start as *mut u8
        } else {
            ptr::null_mut()
        }
    }

    /// Deallocates the given block of memory.
    ///
    /// # Note
    /// Because this is a *Bump Allocator*, **individual deallocations are a no-op**.
    /// Freed memory cannot be reused until the allocator is completely reset.
    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: core::alloc::Layout) {
        // Intentionally empty
    }
}

#[global_allocator]
pub static ALLOCATOR: BumpAllocator = BumpAllocator::new();
