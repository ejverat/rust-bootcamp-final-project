#![cfg_attr(not(test), no_std)]

pub mod testing {
    extern crate alloc;

    use core::fmt::Debug;

    use alloc::{borrow::ToOwned, boxed::Box, format, string::String, vec::Vec};
    use cortex_m_semihosting::hprintln;

    pub struct TestInfo {
        title: String,
        function: Box<dyn Fn() -> (String, bool) + 'static>,
        expected: String,
    }

    pub struct TestResult {
        title: String,
        test_result: String,
        expected: String,
        passed: bool,
    }

    impl Debug for TestResult {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            if self.passed {
                write!(f, "{}: PASSED  ", &self.title)
            } else {
                write!(
                    f,
                    "{}: FAILED\n  EXPECTED: {}\n  RESULT:   {}",
                    &self.title, &self.expected, &self.test_result
                )
            }
        }
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

        pub fn assert<F, G, T>(&mut self, f: F, validator: G, value: T)
        where
            F: Fn() -> T + 'static,
            G: Fn(T, T) -> bool + 'static,
            T: Debug + PartialEq + Copy + 'static,
        {
            let test_info = TestInfo {
                title: self.title.to_owned(),
                function: Box::new(move || {
                    let result = f();
                    let compare_result = validator(result, value);
                    (format!("{:?}", result), compare_result)
                }),
                expected: format!("{:?}", value),
            };
            self.runner.tests.push(test_info);
        }

        pub fn assert_eq<F, T>(&mut self, f: F, value: T)
        where
            F: Fn() -> T + 'static,
            T: Debug + PartialEq + Copy + 'static,
        {
            self.assert(f, |x, y| x == y, value);
        }

        pub fn assert_neq<F, T>(&mut self, f: F, value: T)
        where
            F: Fn() -> T + 'static,
            T: Debug + PartialEq + Copy + 'static,
        {
            self.assert(f, |x, y| x != y, value);
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
                    test_result: result.0,
                    expected: t.expected.to_owned(),
                    passed: result.1,
                };

                hprintln!("{:?}", test_result);
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
