#![no_std]

use core::cell::Cell;
use core::ptr;
use libtock_platform as platform;
use libtock_platform::allow_ro::AllowRo;
use libtock_platform::subscribe::Subscribe;
use platform::{
    share::{self},
    DefaultConfig, ErrorCode, Syscalls,
};

// we don't need to worry about data races as Tock operates on single thread
static mut INITED: bool = false;

// fill app_state section with enough space
#[no_mangle]
#[link_section = ".app_state"]
static mut FLASH_BUFFER: [u8; 20] = [0u8; 20];
static mut FLASH_PTR: *mut u32 = ptr::null_mut();

pub struct AppState<
    T: Sized,
    S: Syscalls,
    C: platform::allow_ro::Config + platform::subscribe::Config = DefaultConfig,
>(T, S, C);

impl<S: Syscalls, C: platform::allow_ro::Config + platform::subscribe::Config, T: Sized>
    AppState<T, S, C>
{
    // the app_flash driver capsule invokes the non-volatile storage driver to write data to flash
    // thus need to check both drivers
    #[inline(always)]
    pub fn driver_check() -> bool {
        let ret = S::command(DRIVER_NUM, command::DRIVER_CHECK, 0, 0).is_success()
            & S::command(NVM_STORAGE_NUM, command::DRIVER_CHECK, 0, 0).is_success();
        ret
    }

    pub fn init(ram_ptr: *mut T) -> Result<(), ErrorCode> {
        // convert raw pointer to byte buffer reference
        let ram_buffer =
            unsafe { core::slice::from_raw_parts(ram_ptr as *const u8, core::mem::size_of::<T>()) };
        share::scope::<(AllowRo<_, DRIVER_NUM, { allow_ro::WRITE_FLASH }>,), _, _>(|handle| {
            let (allow_ro,) = handle.split();

            S::allow_ro::<C, DRIVER_NUM, { allow_ro::WRITE_FLASH }>(allow_ro, ram_buffer)?;

            let n = S::memop_number_writeable_flash_regions()?;
            if n == 0 {
                return Err(ErrorCode::NoMem);
            }

            //flash_buffer_raw = flash_buffer.as_mut_ptr();
            unsafe {
                FLASH_PTR = S::memop_flash_region_begins_at(0)? as *mut u32;
                INITED = true;
            }
            Ok(())
        })
    }

    // asynchronous function to save to flash memory
    // writes to .app_state section
    // needs to be called inside share::scope function,
    // the call should also be suceeded with _yield call
    pub fn save<'share>(
        ram_ptr: *mut T,
        callback: &'share Cell<Option<(u32,)>>,
        handle: share::Handle<(Subscribe<'share, S, DRIVER_NUM, 0>,)>,
    ) -> Result<(), ErrorCode> {
        unsafe {
            if !INITED {
                AppState::<T, S, C>::init(ram_ptr)?;
            }
        }

        let (subscribe,) = handle.split();

        S::subscribe::<_, _, C, DRIVER_NUM, { subscribe::SUBSCRIBE_WRITE }>(subscribe, callback)?;
        // flash_ptr is ignored, no value written in fake implementation,
        // loading it makes no sense, need faked save
        // we pass adress of static variable defined in memop_impl.rs
        unsafe {
            S::command(DRIVER_NUM, command::WRITE_FLASH, FLASH_PTR as u32, 0).to_result()?;
        }
        Ok(())
    }
    // Write flash memory fails because non-volatile storage driver is not working
    pub fn save_sync(ram_ptr: *mut T) -> Result<(), ErrorCode> {
        unsafe {
            if !INITED {
                AppState::<T, S, C>::init(ram_ptr)?;
            }
        }
        let called = core::cell::Cell::new(Option::<(u32,)>::None);
        let ret = share::scope::<(Subscribe<_, DRIVER_NUM, { subscribe::SUBSCRIBE_WRITE }>,), _, _>(
            |handle| {
                let (subscribe,) = handle.split();

                S::subscribe::<_, _, C, DRIVER_NUM, { subscribe::SUBSCRIBE_WRITE }>(
                    subscribe, &called,
                )?;

                // if we run tests, we get lossy cast from 64 bit address to 32 bit number, which may explain why the tests fail the save
                unsafe {
                    S::command(DRIVER_NUM, command::WRITE_FLASH, FLASH_PTR as u32, 0)
                        .to_result()?;
                }
                S::yield_wait();
                if let Some((_,)) = called.get() {
                    return Ok(());
                }
                Err(ErrorCode::Fail)
            },
        );

        ret
    }

    pub unsafe fn load_sync(ram_ptr: *mut T) -> Result<(), ErrorCode> {
        if !INITED {
            AppState::<T, S, C>::init(ram_ptr)?;
        }

        ptr::write_unaligned(ram_ptr, ptr::read(FLASH_PTR as *mut T));
        Ok(())
    }

    // wait for asynchronous functions to finish
    pub fn _yield<'share>(callback: &'share Cell<Option<(u32,)>>) -> Result<(), ErrorCode> {
        S::yield_wait();
        match (*callback).get() {
            Some((_,)) => Ok(()),
            _ => Err(ErrorCode::Fail),
        }
    }
}

#[cfg(test)]
mod tests;
const DRIVER_NUM: u32 = 0x50000;
const NVM_STORAGE_NUM: u32 = 0x50001;

// Command IDs
#[allow(unused)]
mod command {
    pub const DRIVER_CHECK: u32 = 0;
    pub const WRITE_FLASH: u32 = 1;
}

#[allow(unused)]
mod subscribe {
    use libtock_platform::subscribe;
    pub const SUBSCRIBE_WRITE: u32 = 0;
}

mod allow_ro {
    pub const WRITE_FLASH: u32 = 0;
}
