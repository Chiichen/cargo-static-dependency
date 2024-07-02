#![no_std]

use core::alloc::GlobalAlloc;

pub fn add(left: u64, right: u64) -> u64 {
    static_dep::add(left, right)
}

pub struct TestAllocator;

#[global_allocator]
static ALLOCATOR: TestAllocator = TestAllocator;

unsafe impl GlobalAlloc for TestAllocator {
    unsafe fn alloc(&self, layout: core::alloc::Layout) -> *mut u8 {
        todo!()
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: core::alloc::Layout) {
        todo!()
    }
}
