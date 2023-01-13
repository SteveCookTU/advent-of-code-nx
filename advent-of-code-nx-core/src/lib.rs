#![no_std]

mod advent_2022;

extern crate alloc;

use alloc::ffi::CString;
use alloc::format;
use core::ffi::{c_char, CStr};
use core::panic::PanicInfo;
use linked_list_allocator::LockedHeap;

#[doc(hidden)]
#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> ! {
    loop {}
}

const FAKE_HEAP_SIZE: usize = 0x100000;

#[repr(align(0x1000))]
struct AlignedFakeHeapContainer([u8; FAKE_HEAP_SIZE]);

static mut FAKE_HEAP: AlignedFakeHeapContainer = AlignedFakeHeapContainer([0; FAKE_HEAP_SIZE]);

#[global_allocator]
static ALLOCATOR: LockedHeap = LockedHeap::empty();

#[no_mangle]
pub unsafe extern "C" fn init_heap() {
    let fake_heap_ptr = FAKE_HEAP.0.as_mut_ptr();
    ALLOCATOR
        .lock()
        .init(fake_heap_ptr, fake_heap_ptr.add(FAKE_HEAP_SIZE) as usize);
}

#[no_mangle]
pub extern "C" fn run_day(year: i32, day: i32, input: *const c_char) -> *mut c_char {
    let c_str = unsafe { CStr::from_ptr(input) };
    let input = c_str.to_str().unwrap_or("");

    let result = match year {
        2022 => advent_2022::run_day(day, input),
        _ => format!("Year {} is not yet available!", year),
    };

    CString::new(result).unwrap_or_default().into_raw()
}

#[no_mangle]
pub extern "C" fn free_result(s: *mut c_char) {
    unsafe {
        if s.is_null() {
            return;
        }
        CString::from_raw(s)
    };
}
