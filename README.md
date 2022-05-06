![Build Status](https://github.com/tock/libtock-rs/workflows/ci/badge.svg)

# libtock-rs

Rust userland library for Tock

## Getting Started

The experiments for thesis were run using microbit_v2 board. The apps that I worked on are rng, app_state located in libtock2/examples folder.

1.  Ensure you have [rustup](https://www.rustup.rs/) installed.

1.  Clone the repository:

    ```shell
    git clone --recursive https://github.com/tock/libtock-rs
    cd libtock-rs
    ```

1.  Install the dependencies:

    ```shell
    make setup
    ```
    
1.  Use `make` to build examples:

    ```bash
    make flash-microbit_v2 EXAMPLE=rng # Flash the example 'rng' program to microbit_v2 platform
    ```
1.  Use `make` to run tests. Make sure you have rustc version 1.59.0 or higher:
    ```bash
    make test
    ```
## Project structure 
Here are some pointers to better navigate project structure.

libtock2/examples - location of Tock apps
apis/ - userspace libraries
platform/ - system call interfaces
runner/ - runner CLI app that is used to help building executables and passing to tockloader
runtime/ - layouts for different boards and system call implementations for ARM and RISC-V architectures
syscall_tests/ - tests for system calls
unittest/ - fake kernel and driver(capsule) implementations
tock/ - submodule for Tock kernel (set on version release-2.0)

## Using libtock-rs

This script does the following steps for you:

- cross-compile your program
- create a TAB (tock application bundle)
- if you have a J-Link compatible board connected: flash this TAB to your board (using tockloader)
