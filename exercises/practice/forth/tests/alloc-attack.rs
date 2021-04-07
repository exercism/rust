//! Certain implementations of Forth naively and eagerly expand sub-definitions
//! into superior definitions at definition time. While this appears at first to
//! be a clever trick to solve the problem imposed by the
//! `can_use_different_words_with_the_same_name` test, it has its own problem: it
//! can be attacked to cause an OOM condition in an arbitrary machine without
//! requiring that any instructions are actually executed; only definitions.
//!
//! This test doesn't go quite that far, but it does demonstrate the attack and
//! require that implementations don't just do the naive thing.

use forth::Forth;

#[test]
#[ignore]
fn alloc_attack() {
    let mut f = Forth::new();
    f.eval(": a 0 drop ;").unwrap();
    f.eval(": b a a ;").unwrap();
    f.eval(": c b b ;").unwrap();
    f.eval(": d c c ;").unwrap();
    f.eval(": e d d ;").unwrap();
    f.eval(": f e e ;").unwrap();
    f.eval(": g f f ;").unwrap();
    f.eval(": h g g ;").unwrap();
    f.eval(": i h h ;").unwrap();
    f.eval(": j i i ;").unwrap();
    f.eval(": k j j ;").unwrap();
    f.eval(": l k k ;").unwrap();
    f.eval(": m l l ;").unwrap();
    f.eval(": n m m ;").unwrap();
    f.eval(": o n n ;").unwrap();
    f.eval(": p o o ;").unwrap();
    f.eval(": q p p ;").unwrap();
    f.eval(": r q q ;").unwrap();
    f.eval(": s r r ;").unwrap();
    f.eval(": t s s ;").unwrap();
    f.eval(": u t t ;").unwrap();
    f.eval(": v u u ;").unwrap();
    f.eval(": w v v ;").unwrap();
    f.eval(": x w w ;").unwrap();
    f.eval(": y x x ;").unwrap();
    f.eval(": z y y ;").unwrap();

    // On an implementation with eager expansion of sub-custom-words,
    // `z`'s definition is 2**26 items long. Assuming the implementation
    // has compacted each instruction into a single byte, that takes up
    // over 50 Mb of memory. Less efficient implementations will require
    // more.
    //
    // This shouldn't crash or hang anyone's machine, but it's at least a
    // testable proposition. A good implementation shouldn't be doing eager
    // stack expansion, so it should require much less than that.

    // Sanity check--few implementations should fail here.
    assert!(f.stack().is_empty());

    // We want to allow some wiggle-room, as this exercise isn't really meant
    // to test the ability to micro-optimize. Still, let's ensure that the total
    // allocated memory is substantially less than expected from a naive solution.
    //
    // A megabyte seems like a reasonable number to use.
    assert!(GLOBAL_ALLOCATOR.get_bytes_allocated() < 1024 * 1024);
}

// From this point forward, we define the custom `GLOBAL_ALLOCATOR` used in the test above.

use std::alloc::{GlobalAlloc, Layout, System as SystemAllocator};
use std::sync::atomic::{AtomicU64, Ordering};

/// This allocator wraps the default allocator, and keeps track of how much
/// memory has been allocated.
///
/// Inspired by https://www.reddit.com/r/rust/comments/8z83wc/is_there_any_way_to_benchmark_memory_usage_in_rust/e2h4dp9
struct TrackingAllocator<A: GlobalAlloc>(A, AtomicU64);

unsafe impl<A: GlobalAlloc> GlobalAlloc for TrackingAllocator<A> {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        self.1.fetch_add(layout.size() as u64, Ordering::SeqCst);
        self.0.alloc(layout)
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        self.0.dealloc(ptr, layout);
        self.1.fetch_sub(layout.size() as u64, Ordering::SeqCst);
    }
}

impl<A: GlobalAlloc> TrackingAllocator<A> {
    fn get_bytes_allocated(&self) -> u64 {
        self.1.load(Ordering::SeqCst)
    }
}

#[global_allocator]
static GLOBAL_ALLOCATOR: TrackingAllocator<SystemAllocator> =
    TrackingAllocator(SystemAllocator, AtomicU64::new(0));
