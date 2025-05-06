#![no_std]
#![no_main]

use core::mem::MaybeUninit;

use alloc::format;
// pick a panicking behavior
use panic_halt as _; // you can put a breakpoint on `rust_begin_unwind` to catch panics
                     // use panic_abort as _; // requires nightly
                     // use panic_itm as _; // logs messages over ITM; requires ITM support
                     // use panic_semihosting as _; // logs messages to the host stderr; requires a debugger

use cortex_m_rt::entry;
use cortex_m_semihosting::{debug, hprintln};

use embedded_alloc::LlffHeap as Heap;

extern crate alloc;

use embedded_test_runner::testing;

#[global_allocator]
static HEAP: Heap = Heap::empty();

fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[entry]
fn main() -> ! {
    {
        const HEAP_SIZE: usize = 32 * 1024;

        static mut HEAP_MEM: [MaybeUninit<u8>; HEAP_SIZE] = [MaybeUninit::uninit(); HEAP_SIZE];
        unsafe { HEAP.init(HEAP_MEM.as_ptr() as usize, HEAP_SIZE) }
    }
    hprintln!("Test runner");

    let mut test_runner = testing::Runner::new();

    for i in 0..50 {
        let x = i;
        let y = 2 * i;
        let valid_result = x + y;
        test_runner
            .test(format!("add({},{}) = {}", x, y, valid_result).as_str())
            .assert_eq(move || add(x, y), valid_result);

        test_runner
            .test(format!("add({},{}) != {}", x, y, valid_result).as_str())
            .assert_neq(move || add(x, y), valid_result);
    }

    let _test_results = test_runner.run();

    // In case we want a personalized of reporter format
    // testing::report_test(&test_results);

    debug::exit(debug::EXIT_SUCCESS);

    loop {
        // your code goes here
    }
}
