
# libtock-rs

Rust userland library for Tock

## Getting Started

The experiments for thesis were run using BBC micro:bit board v2.20 (microbit_v2) board, so you would need one to see them. Here are instructions to make them work:

1.  Ensure that you have flashed Tock kernel on your microbit_v2 board. If not, follow this [guide](https://github.com/tock/tock/tree/0e91e3ed8338b6b7dd0603c76a63afe25429febe/boards/microbit_v2).

3.  Ensure you have [rustup](https://www.rustup.rs/) installed.

1.  Clone the repository:

    ```shell
    git clone -b thesis --recursive https://github.com/tock/libtock-rs
    cd libtock-rs
    ```

1.  Install the dependencies:

    ```shell
    make setup
    ```
    
1.  Use `make` to build examples and run on microbit board:

    ```bash
    make flash-microbit_v2 EXAMPLE=rng # Flash the example 'rng' program to microbit_v2 platform
    ```
1.  Use `make` to run tests. Make sure you have rustc version [1.59.0 or higher](https://github.com/tock/libtock-rs/issues/394#issuecomment-1064336779):
    ```bash
    make test
    ```
    The test script will do following things: run the tests in normal mode, check the code formatting using ```cargo fmt --check```, run the linter clippy for different target platforms, and run tests in Miri. Miri helps to detect classes of undefined behavior and memory leaks, for more information visit [Miri project page](https://github.com/rust-lang/miri)  
## Project structure 
Here are some folder pointers to better navigate project structure.

- libtock2/examples - location of Tock apps
- apis/ - userspace libraries
- platform/ - system call interfaces
- runner/ - runner CLI app that is used to help building executables and passing to tockloader
- runtime/ - layouts for different boards and system call implementations for ARM and RISC-V architectures
- syscall_tests/ - tests for system calls
- unittest/ - fake kernel and driver(capsule) implementations
- tock/ - submodule for Tock kernel (set to version release-2.0)
## Our work

- apis/rng - rng library (with tests)
- apis/app\_state - app\_state library (with tests)
- libtock2/examples/rng.rs - example application using rng library
- libtock2/examples/app\_state.rs - example application using app\_state library
- platform/src/syscalls.rs - declaration of memop system call
- platform/src/syscalls\_impl.rs - implementation of memop system call
- runtime/libtock\_layout.ld - process layout of libtock-rs application
- runtime/layouts/microbit\_v2.ld - layout for microbit board
- unittest/src/fake/app\_state - fake implementation of app\_state driver
- unittest/src/fake/rng - fake implementation of rng driver, used in tests
- unittest/src/fake/app\_state - fake implementation of app\_state driver, used in tests
- syscalls\_tests/src/memop.rs - tests for system call memop

## Using libtock-rs

This script does the following steps for you:

- cross-compile your program
- create a TAB (tock application bundle)
- if you have a J-Link compatible board connected: flash this TAB to your board (using tockloader)
