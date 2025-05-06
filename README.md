# Rust bootcamp project

As part of the rust developer bootcamp from <https://portal.letsgetrusty.com/>,
this repo contains my project source code.

# Project description

Test runner implementation with targeting to run in an no std embedded
environment. In my particular case, I decided  to run it in a cortex-M3
cpu using the QEMU emulator tool.

The project contains a library crate
([lib.rs](./embedded-test-runner/src/lib.rs) )
with the implementation of the test runner and a binary crate
([main.rs](./embedded-test-runner/src/main.rs)) to test the
functionality of the library.

## How to run

```sh
rustup target add thumbv7m-none-eabi # one time execution
cargo run

```

## Usage example

```rust

use embedded_test_runner::testing;

/*
  ...
 */

fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[entry]
fn main() -> ! {

  /*
    ...
   */

  let mut test_runner = testing::Runner::new();

  test_runner
      .test("Passing test")
      .assert_eq(|| add(5, 5), 10);

  test_runner
      .test("Failing test")
      .assert_neq(|| add(5, 5), 10);

  test_runner.run();

```

# Library implementation

The implementation of the library is composed by the following structs:

- Runner
- TestBuilder
- TestInfo
- TestResult

## Runner

The runner was implemented thinking as the main interface to start writing test,
and run it.

## TestBuilder

The test builder role is mainly to create a test, this is created by calling the
`Runner::test()` method. The builder contains multiple `assert` methods to create
tests.

## TestInfo

This struct is not public, since it is used only to store the information about
each test created by `TestBuilder`.

## TestResult

Store the output after executing a test, like the result, expected, if test
passed or failed, etc.
