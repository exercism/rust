use doubly_linked_list::*;

// counter of allocated bytes
// used by the allocator defined below the test
static ALLOCATED: AtomicUsize = AtomicUsize::new(0);

// test that all memory is deallocated
// does not check if the destructor is run
#[test]
#[ignore]
fn drop_no_leaks() {
    let allocated_before = ALLOCATED.load(SeqCst);
    let list = (0..10).collect::<LinkedList<_>>();
    let list_allocation_size = ALLOCATED.load(SeqCst) - allocated_before;
    assert!(
        list_allocation_size != 0,
        "no bytes were allocated for a nonempty list"
    );
    drop(list);

    let allocated_after = ALLOCATED.load(SeqCst);
    let leaked_bytes = allocated_before - allocated_after;
    assert!(leaked_bytes == 0);
}

// Defines a wrapper around the global allocator that counts allocations
// to check that no memory has been leaked. Code taken from docs for std::alloc::System.
// Because this is a global allocator, other tests could interfere, so
// the tests have to be run consecutively and not in parallel as they would by default.
use std::alloc::{GlobalAlloc, Layout, System};
use std::sync::atomic::{AtomicUsize, Ordering::SeqCst};

struct Counter;

unsafe impl GlobalAlloc for Counter {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let ret = System.alloc(layout);
        if !ret.is_null() {
            ALLOCATED.fetch_add(layout.size(), SeqCst);
        }
        ret
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        System.dealloc(ptr, layout);
        ALLOCATED.fetch_sub(layout.size(), SeqCst);
    }
}

#[global_allocator]
static A: Counter = Counter;
