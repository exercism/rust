use doubly_linked_list::*;

// counter of allocated bytes
// used by the allocator defined below the test
static ALLOCATED: AtomicUsize = AtomicUsize::new(0);

// test that removing an element via the cursor deallocates memory
// does not check if the destructor is run
#[test]
#[ignore]
fn drop_no_leak_when_removing_single_element() {
    let mut list = (0..10).collect::<LinkedList<_>>();

    let mut cursor = list.cursor_front();
    assert!(cursor.seek_forward(4));

    let allocated_before = ALLOCATED.load(SeqCst);
    cursor.take();
    let allocated_after = ALLOCATED.load(SeqCst);

    // only check that something is deallocated
    // we can't know the exact size of the node structure
    assert!(allocated_before > allocated_after);
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
