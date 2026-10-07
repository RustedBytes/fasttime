//! Guard the allocation-free Rust parsing, arithmetic and formatting paths.
use fasttime::{Date, DateTime, Duration, OffsetDateTime, Time};
use std::alloc::{GlobalAlloc, Layout, System};
use std::cell::Cell;
use std::fmt::{self, Write};
use std::hint::black_box;

thread_local! {
    static COUNT: Cell<Option<usize>> = const { Cell::new(None) };
}

struct CountingAllocator;

fn record() {
    let _ = COUNT.try_with(|count| {
        if let Some(value) = count.get() {
            count.set(Some(value + 1));
        }
    });
}

// SAFETY: all operations delegate unchanged layouts and pointers to System.
unsafe impl GlobalAlloc for CountingAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        record();
        unsafe { System.alloc(layout) }
    }
    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        record();
        unsafe { System.alloc_zeroed(layout) }
    }
    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, size: usize) -> *mut u8 {
        record();
        unsafe { System.realloc(ptr, layout, size) }
    }
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        unsafe { System.dealloc(ptr, layout) }
    }
}

#[global_allocator]
static ALLOCATOR: CountingAllocator = CountingAllocator;

struct Buffer([u8; 128], usize);
impl Write for Buffer {
    fn write_str(&mut self, text: &str) -> fmt::Result {
        let end = self.1 + text.len();
        self.0
            .get_mut(self.1..end)
            .ok_or(fmt::Error)?
            .copy_from_slice(text.as_bytes());
        self.1 = end;
        Ok(())
    }
}

#[test]
fn rust_hot_paths_do_not_allocate() {
    // Positive control proves this thread's allocator instrumentation is active.
    COUNT.with(|count| count.set(Some(0)));
    let allocation = black_box(Box::new(42));
    assert!(COUNT.with(|count| count.replace(None).unwrap()) > 0);
    drop(allocation);

    COUNT.with(|count| count.set(Some(0)));
    for input in ["2024-06-15", "-2147483648-01-01", "2147483647-12-31"] {
        let date = black_box(input).parse::<Date>().unwrap();
        let mut buffer = Buffer([0; 128], 0);
        write!(buffer, "{date}").unwrap();
        black_box(buffer);
    }
    let time = black_box("12:30:45.123456789").parse::<Time>().unwrap();
    let dt = black_box("2024-06-15T12:30:45.123456789Z")
        .parse::<DateTime>()
        .unwrap();
    let offset = black_box("2024-06-15T12:30:45.123456789+05:30")
        .parse::<OffsetDateTime>()
        .unwrap();
    for _ in 0..100 {
        black_box(DateTime::from_unix_timestamp(dt.unix_timestamp(), 123456789).unwrap());
        black_box(dt.add_duration(Duration::seconds(1)).unwrap());
        black_box(offset.to_local().unwrap());
        let mut buffer = Buffer([0; 128], 0);
        write!(buffer, "{time} {dt} {offset}").unwrap();
        black_box(buffer);
    }
    let _ = black_box("invalid".parse::<Date>());
    let _ = black_box("invalid".parse::<Time>());
    let allocations = COUNT.with(|count| count.replace(None).unwrap());
    assert_eq!(allocations, 0);
}
