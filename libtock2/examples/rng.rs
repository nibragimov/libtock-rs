#![no_main]
#![no_std]

use core::cell::Cell;
use core::fmt::Write;
use core::panic;
use libtock2::console::Console;
use libtock2::rng::Rng;
use libtock2::runtime::TockSyscalls;
use libtock2::runtime::{set_main, stack_size};
use libtock_platform::Syscalls;
use libtock_platform::{share, ErrorCode};
use numtoa::NumToA;
set_main! {main}
stack_size! {0x100}

fn main() {
    let mut buffer = [0u8; 3];
    let mut writer = Console::writer();
    let mut num_buffer = [0u8; 10];
    let num: u32 = (buffer.len()) as u32;

    // check if the driver is supported by the board
    if !Rng::driver_check() {
        writeln!(writer, "Driver not supported").unwrap();
        return;
    }

    // Call rng synchronously (commented out)
    // let ret = Rng::gen(&mut buffer, num);

    // using asynchronous API to generate numbers
    //
    // the callback is of the type that implements Upcall trait
    let callback = Cell::new(Option::<(u32,)>::None);
    let ret = share::scope(|handle| {
        Rng::gen_async(&callback, &mut buffer, handle, num)?;
        // waits for the function Rng::gen_async() to finish
        TockSyscalls::yield_wait();
        // check if upcall was invoked
        match callback.get() {
            Some((_,)) => Ok(()),
            _ => Err(ErrorCode::Fail),
        }
    });

    // error handling code
    if let Err(e) = ret {
        writeln!(writer, "Error during generation: ").unwrap();
        if let Some(s) = as_str(e) {
            writeln!(writer, "{}", s).unwrap();
        } else {
            writeln!(writer, "Unknown error").unwrap();
        }
        return;
    }

    writeln!(writer, "Random nums generated: ").unwrap();
    for x in &buffer {
        // numtoa helps to print numbers
        let n = (*x).numtoa_str(10, &mut num_buffer);
        writeln!(writer, "{}", n).unwrap();
    }
}
// helper function for error-handling, maps ErrorCode values to strings
// for printing on the console
fn as_str(e: ErrorCode) -> Option<&'static str> {
    match e {
        ErrorCode::Fail => Some("FAIL"),
        ErrorCode::Busy => Some("BUSY"),
        ErrorCode::Already => Some("ALREADY"),
        ErrorCode::Off => Some("OFF"),
        ErrorCode::Reserve => Some("RESERVE"),
        ErrorCode::Invalid => Some("INVALID"),
        ErrorCode::Size => Some("SIZE"),
        ErrorCode::Cancel => Some("CANCEL"),
        ErrorCode::NoMem => Some("NOMEM"),
        ErrorCode::NoSupport => Some("NOSUPPORT"),
        ErrorCode::NoDevice => Some("NODEVICE"),
        ErrorCode::Uninstalled => Some("UNINSTALLED"),
        ErrorCode::NoAck => Some("NOACK"),
        ErrorCode::BadRVal => Some("BADRVAL"),
        _ => None,
    }
}
