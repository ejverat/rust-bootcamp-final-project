#![cfg_attr(not(test), no_std)]

pub mod testing {
    extern crate alloc;

    use core::fmt::Debug;

    use alloc::{borrow::ToOwned, boxed::Box, format, string::String, vec::Vec};
    use cortex_m_semihosting::hprintln;

    /// Holds the test information. This is used internally by `Runner`
    struct TestInfo {
        title: String,
        function: Box<dyn Fn() -> (String, bool) + 'static>,
        expected: String,
    }

    /// This structs is used to store the output of the test execution.
    /// This is public since it is thinked to be used with a custom test reporter.
    ///
    /// Fields:
    /// - `title` stores the test title
    /// - `test_result` stores the returned value from user closure converted to string
    /// - `expected` stores the expected value converted to string
    /// - `passed` indicates if test passed or failed
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

    /// Stores the tests created by the user. And is used as interface
    /// to start to create and run tests.
    pub struct Runner {
        tests: Vec<TestInfo>,
    }

    /// Holds the instance of the Runner who create this and test title.
    /// The main goal is to help to create test by using its asserts methods
    pub struct TestBuilder<'a> {
        runner: &'a mut Runner,
        title: String,
    }

    impl<'a> TestBuilder<'a> {
        /// Creates a new `TestBuilder` with the specified title
        /// and reference to the `Runner`.
        fn new(runner: &'a mut Runner, title: &str) -> Self {
            TestBuilder {
                runner,
                title: title.to_owned(),
            }
        }

        /// Creates and store a test inside the Runner
        ///
        /// Fields:
        /// - `f` is the user closure that contains the test logic implementation.
        /// - `validator` is a closure that must be used to compare the output of `f` with the
        /// expected value.
        /// - `expected_value` is the value that is expected from the output of `f`
        ///
        /// # Example
        /// ```
        /// use embedded_test_runner::testing;
        ///
        /// let mut runner = testing::Runner::new();
        /// runner.test("example").assert(|| { 5 + 3}, |a,b|{a == b}, 8);
        ///
        /// ```
        pub fn assert<F, G, T>(&mut self, f: F, validator: G, expected_value: T)
        where
            F: Fn() -> T + 'static,
            G: Fn(T, T) -> bool + 'static,
            T: Debug + PartialEq + Copy + 'static,
        {
            let test_info = TestInfo {
                title: self.title.to_owned(),
                function: Box::new(move || {
                    let result = f();
                    let compare_result = validator(result, expected_value);
                    (format!("{:?}", result), compare_result)
                }),
                expected: format!("{:?}", expected_value),
            };
            self.runner.tests.push(test_info);
        }

        /// Creates and store a test inside the container of the Runner by specializing the assert
        /// and comparing if the test result is equal to the expected value.
        ///
        /// Fields:
        /// - `f` is the user closure that contains the test logic implementation.
        /// - `expected_value` is the value that is expected from the output of `f`
        ///
        /// # Example
        /// ```
        /// use embedded_test_runner::testing;
        ///
        /// let mut runner = testing::Runner::new();
        /// runner.test("example").assert_eq(|| { 5 + 3}, 8);
        ///
        /// ```
        pub fn assert_eq<F, T>(&mut self, f: F, expected_value: T)
        where
            F: Fn() -> T + 'static,
            T: Debug + PartialEq + Copy + 'static,
        {
            self.assert(f, |x, y| x == y, expected_value);
        }

        /// Creates and store a test inside the container of the Runner by specializing the assert
        /// and comparing if the test result is not equal to the expected value.
        ///
        /// Fields:
        /// - `f` is the user closure that contains the test logic implementation.
        /// - `expected_value` is the value that is expected from the output of `f`
        ///
        /// # Example
        /// ```
        /// use embedded_test_runner::testing;
        ///
        /// let mut runner = testing::Runner::new();
        /// runner.test("example").assert_neq(|| { 5 + 4}, 8);
        ///
        /// ```
        pub fn assert_neq<F, T>(&mut self, f: F, value: T)
        where
            F: Fn() -> T + 'static,
            T: Debug + PartialEq + Copy + 'static,
        {
            self.assert(f, |x, y| x != y, value);
        }
    }

    impl Runner {
        /// Create a new Runner instance to start creating and running tests
        ///
        /// # Example
        /// ```
        /// use embedded_test_runner::testing;
        ///
        /// let mut runner = testing::Runner::new();
        ///
        /// ```
        pub fn new() -> Self {
            Self { tests: Vec::new() }
        }

        /// Create a test builder to start creating tests. Only the `title` of test is required
        ///
        /// # Example
        /// ```
        /// use embedded_test_runner::testing;
        ///
        /// let mut runner = testing::Runner::new();
        /// runner.test("example").assert_neq(|| { 5 + 4}, 8);
        ///
        /// ```
        pub fn test(&mut self, title: &str) -> TestBuilder {
            TestBuilder::new(self, title)
        }

        /// Execute the scheduled tests and returns the result as a `Vec<TestResult>` with the
        /// purpose of create an specialized test reporter or any other after processing. Currently
        /// this code displays in real time the results per each test.
        ///
        /// # Example
        /// ```
        /// use embedded_test_runner::testing;
        ///
        /// let mut runner = testing::Runner::new();
        /// runner.test("example").assert_eq(|| { 5 + 3}, 8);
        /// runner.test("example").assert_neq(|| { 5 + 3}, 9);
        ///
        /// let _test_results = runner.run();
        ///
        /// ```
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

    /// Test reporter example 1
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

    /// Test reporter example 2
    pub fn report_test_debug(test_results: &Vec<TestResult>) {
        for result in test_results {
            hprintln!("{:?}", result);
        }
    }
}
