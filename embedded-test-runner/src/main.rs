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

#[global_allocator]
static HEAP: Heap = Heap::empty();

pub mod testing {
    use core::fmt::Debug;

    use alloc::{borrow::ToOwned, boxed::Box, format, string::String, vec::Vec};
    use cortex_m_semihosting::hprintln;

    pub struct TestInfo {
        title: String,
        function: Box<dyn Fn() -> String + 'static>,
        expected: String,
    }

    #[derive(Debug)]
    pub struct TestResult {
        title: String,
        test_result: String,
        expected: String,
        passed: bool,
    }

    pub struct Runner {
        tests: Vec<TestInfo>,
    }

    pub struct TestBuilder<'a> {
        runner: &'a mut Runner,
        title: String,
    }

    impl<'a> TestBuilder<'a> {
        pub fn new(runner: &'a mut Runner, title: &str) -> Self {
            TestBuilder {
                runner,
                title: title.to_owned(),
            }
        }

        pub fn assert_eq<F, T>(&mut self, f: F, value: &T)
        where
            F: Fn() -> T + 'static,
            T: Debug + PartialEq,
        {
            let test_info = TestInfo {
                title: self.title.to_owned(),
                function: Box::new(move || format!("{:?}", f())),
                expected: format!("{:?}", value),
            };
            self.runner.tests.push(test_info);
        }
    }

    impl Runner {
        pub fn new() -> Self {
            Self { tests: Vec::new() }
        }

        pub fn test(&mut self, title: &str) -> TestBuilder {
            TestBuilder::new(self, title)
        }

        pub fn run(&mut self) -> Vec<TestResult> {
            let mut tests_results: Vec<TestResult> = Vec::new();
            for t in &mut self.tests {
                let result = t.function.as_ref()();
                let test_result = TestResult {
                    title: t.title.to_owned(),
                    test_result: result.to_owned(),
                    expected: t.expected.to_owned(),
                    passed: t.expected == result,
                };

                tests_results.push(test_result);
            }

            tests_results
        }
    }

    impl Default for Runner {
        fn default() -> Self {
            Self::new()
        }
    }

    pub fn report_test(test_results: &Vec<TestResult>) {
        let num_failed_tests = test_results
            .iter()
            .filter(|t| t.passed == false)
            .map(|ft| {
                hprintln!("{}: FAILED", ft.title);
                hprintln!("EXPECTED: {}", ft.expected);
                hprintln!("RESULT:   {}", ft.test_result);
            })
            .count();

        if num_failed_tests == 0 {
            hprintln!("ALL TEST PASSED");
        } else {
            hprintln!(
                "PASSED: {} - FAILED: {}",
                test_results.len() - num_failed_tests,
                num_failed_tests
            );
        }
    }

    pub fn report_test_debug(test_results: &Vec<TestResult>) {
        for result in test_results {
            hprintln!("{:?}", result);
        }
    }
}

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
        .test("Passing test")
        .assert_eq(|| add(5, 5), &10);

    test_runner.test("Failing test").assert_eq(|| add(3, 3), &7);

    let test_results = test_runner.run();

    testing::report_test(&test_results);

    debug::exit(debug::EXIT_SUCCESS);

    loop {
        // your code goes here
    }
}
