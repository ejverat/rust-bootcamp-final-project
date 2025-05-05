#![no_std]
#![no_main]

use core::mem::MaybeUninit;

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
        const HEAP_SIZE: usize = 4 * 1024;

        static mut HEAP_MEM: [MaybeUninit<u8>; HEAP_SIZE] = [MaybeUninit::uninit(); HEAP_SIZE];
        unsafe { HEAP.init(HEAP_MEM.as_ptr() as usize, HEAP_SIZE) }
    }
    hprintln!("Test runner");

    let mut test_runner = testing::Runner::new();

    test_runner
        .test("add(5,5) = 10")
        .assert_eq(|| add(5, 5), &10);

    test_runner
        .test("add(3,7) != 11")
        .assert_neq(|| add(3, 7), &11);

    test_runner
        .test("add(3,3) == 7")
        .assert_eq(|| add(3, 3), &7);

    let _test_results = test_runner.run();

    // In case we want a personalized of reporter format
    // testing::report_test(&test_results);

    debug::exit(debug::EXIT_SUCCESS);

    loop {
        // your code goes here
    }
}
