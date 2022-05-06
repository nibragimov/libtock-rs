#![no_main]
#![no_std]

use core::cell::Cell;
use core::fmt::Write;
use core::ptr;
use libtock2::app_state::AppState;
use libtock2::console::Console;
use libtock2::runtime::{set_main, stack_size};
use libtock_platform::{share, ErrorCode};
use numtoa::NumToA;

set_main! {main}
stack_size! {0x100}

fn try_run(ram_ptr: *mut u32) -> Result<(), ErrorCode> {
    AppState::save_sync(ram_ptr)?;
    unsafe {
        AppState::load_sync(ram_ptr)?;
    }
    Ok(())
}

fn main() {
    let mut writer = Console::writer();
    let mut num = 42u32;
    let mut num_buffer = [0u8; 10];
    let ram_ptr: *mut u32 = &mut num as *mut u32;

    let size = core::mem::size_of::<u32>();
    let callback = Cell::new(Option::<(u32,)>::None);

    // driver check fails, non-volatile storage driver is not working
    if !AppState::driver_check() {
        writeln!(writer, "Driver not supported").unwrap();
        return;
    }

    let ret = try_run(ram_ptr);
    
    // error handling
    if let Err(e) = ret {
        writeln!(writer, "Error: ").unwrap();
        if let Some(s) = as_str(e) {
            writeln!(writer, "{}", s).unwrap();
        } else {
            writeln!(writer, "Unknown error").unwrap();
        }
        return;
    }

    let x: u32 = unsafe { ptr::read(ram_ptr) };
    let n = x.numtoa_str(10, &mut num_buffer);
    writeln!(writer, "{}", n).unwrap();
}

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
