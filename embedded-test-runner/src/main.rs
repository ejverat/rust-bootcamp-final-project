#![no_std]
#![no_main]

use core::mem::MaybeUninit;

use alloc::borrow::ToOwned;
// pick a panicking behavior
use panic_halt as _; // you can put a breakpoint on `rust_begin_unwind` to catch panics
                     // use panic_abort as _; // requires nightly
                     // use panic_itm as _; // logs messages over ITM; requires ITM support
                     // use panic_semihosting as _; // logs messages to the host stderr; requires a debugger

use cortex_m_rt::entry;
use cortex_m_semihosting::{debug, hprintln};

use embedded_alloc::LlffHeap as Heap;
use testing::ExecutableTest;
// use heapless;
//
extern crate alloc;

#[global_allocator]
static HEAP: Heap = Heap::empty();

// pub mod tests {
//     use alloc::{
//         borrow::ToOwned,
//         boxed::Box,
//         collections::btree_map::BTreeMap,
//         rc::Rc,
//         string::{String, ToString},
//         vec::{self, Vec},
//     };
//     use cortex_m_semihosting::hprintln;
//
//     pub fn describe(title: &str, handler: impl Fn()) {
//         hprintln!("{}", title);
//         TESTS_MAP.insert(key, value)
//         handler();
//     }
//
//     pub fn it(title: &str, handler: impl Fn()) {
//         hprintln!("{}", title);
//         handler();
//     }
//
//     enum TestType {
//         Suite,
//         Test(Rc<dyn Fn() + 'static>),
//     }
//
//     struct TestInfo {
//         title: String,
//         test_type: TestType,
//     }
//
//     static mut TESTS_MAP: BTreeMap<String, TestInfo> = BTreeMap::new();
// }

pub mod testing {
    use core::{cell::RefCell, fmt::Debug};

    use alloc::{
        borrow::ToOwned,
        boxed::Box,
        rc::{Rc, Weak},
        string::String,
        vec::Vec,
    };
    use cortex_m_semihosting::hprintln;

    pub trait ExecutableTest {
        fn execute(&mut self);
        fn assert<F: Fn() -> bool + 'static>(&mut self, f: F);
        fn get_result(&self) -> bool;
    }

    pub struct TestItem {
        pub title: String,
        pub function: Box<dyn Fn() -> bool>,
        passed: bool,
    }

    impl Debug for TestItem {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            f.debug_struct("TestItem")
                .field("title", &self.title)
                .field("function", &"Fn()".to_owned())
                .field("result", &self.passed)
                .finish()
        }
    }

    impl ExecutableTest for TestItem {
        fn execute(&mut self) {
            self.passed = self.function.as_ref()();
        }

        fn assert<F: Fn() -> bool + 'static>(&mut self, f: F) {
            self.function = Box::new(f);
        }

        fn get_result(&self) -> bool {
            self.passed
        }
    }

    pub struct Runner {
        tests: Vec<TestItem>,
    }

    impl Runner {
        pub fn new() -> Self {
            Self { tests: Vec::new() }
        }

        pub fn test(&mut self, title: &str) -> &mut TestItem {
            let new_test = TestItem {
                title: title.to_owned(),
                function: Box::new(|| false),
                passed: false,
            };
            self.tests.push(new_test);
            self.tests.last_mut().unwrap()
        }

        pub fn run(&mut self) {
            for t in &mut self.tests {
                t.execute();
                hprintln!("{:?}", t);
            }
        }
    }

    impl Default for Runner {
        fn default() -> Self {
            Self::new()
        }
    }
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

    test_runner.test("First").assert(|| {
        hprintln!("Inside closure");

        6 == 5
    });

    test_runner.test("Second").assert(|| {
        hprintln!("Inside second closure");
        true
    });

    test_runner.run();

    // tests::describe("First", || {
    //     tests::describe("Second", || {
    //         tests::it("Test", || {
    //             hprintln!("Test execution");
    //         });
    //     });
    //     tests::describe("Third", || {
    //         tests::it("Test 2", || {
    //             hprintln!("Second test");
    //         });
    //     });
    // });

    debug::exit(debug::EXIT_SUCCESS);

    loop {
        // your code goes here
    }
}
