#![no_std]

use core::cell::Cell;

use libtock_platform as platform;
use libtock_platform::allow_rw::AllowRw;
use libtock_platform::subscribe::Subscribe;
use platform::{
    share::{self},
    DefaultConfig, ErrorCode, Syscalls,
};

/// The rng driver library.
///
/// It passes buffer to kernel to be filled with random bytes.
///
/// # Example
/// ```ignore
/// use libtock2::Rng;
///
/// // Fills buffer with random bytes
/// let mut buffer = [0u8; 3];
/// let num = buffer.len() as u32;
/// let ret = Rng::gen(&mut buffer, num);
///
/// ```

pub struct Rng<
    S: Syscalls,
    C: platform::allow_rw::Config + platform::subscribe::Config = DefaultConfig,
>(S, C);

impl<S: Syscalls, C: platform::allow_rw::Config + platform::subscribe::Config> Rng<S, C> {
    /// Run a check against the rng capsule to ensure it is present.
    ///
    /// Returns `true` if the driver was present. This does not necessarily mean
    /// that the driver is working, as it may still fail to allocate grant
    /// memory.
    #[inline(always)]
    pub fn driver_check() -> bool {
        S::command(DRIVER_NUM, command::DRIVER_CHECK, 0, 0).is_success()
    }

    // synchronous function that writes 'num' random bytes to buffer
    pub fn gen(buffer: &mut [u8], num: u32) -> Result<(), ErrorCode> {
        if (buffer.len() as u32) < num {
            return Err(ErrorCode::BadRVal);
        }
        // define callback type that implements Upcall trait, visit [platform/src/subscribe.rs]
        let called = core::cell::Cell::new(Option::<(u32,)>::None);
        share::scope::<
            (
                AllowRw<_, DRIVER_NUM, { allow_rw::ALLOW_GEN }>,
                Subscribe<_, DRIVER_NUM, { subscribe::SUBSCRIBE_GEN }>,
            ),
            _,
            _,
        >(|handle| {
            // get handles from a tuple of the type specified above AllowRw<...>, Subscribe<...>
            let (allow_rw, subscribe) = handle.split();
            // share mutable buffer with kernel
            S::allow_rw::<C, DRIVER_NUM, { allow_rw::ALLOW_GEN }>(allow_rw, buffer)?;
            // register uppcall for random number generation
            S::subscribe::<_, _, C, DRIVER_NUM, { subscribe::SUBSCRIBE_GEN }>(subscribe, &called)?;
            // tell kernel to execute function
            S::command(DRIVER_NUM, command::GEN, num, 0).to_result()?;
            // wait unitl upcall is invoked
            S::yield_wait();
            if let Some((_,)) = called.get() {
                return Ok(());
            }
            Err(ErrorCode::Fail)
        })
    }
    // asynchronous function that writes 'num' random bytes to buffer
    // needs to be called inside share::scope function
    // similar to gen, but the waiting is done in the app, gives opportunity
    // to specify own callback (or upcall)
    pub fn gen_async<'share>(
        callback: &'share Cell<Option<(u32,)>>,
        buffer: &'share mut [u8],
        handle: share::Handle<(
            AllowRw<'share, S, DRIVER_NUM, 0>,
            Subscribe<'share, S, DRIVER_NUM, 0>,
        )>,
        num: u32,
    ) -> Result<(), ErrorCode> {
        if (buffer.len() as u32) < num {
            return Err(ErrorCode::BadRVal);
        }
        let (allow_rw, subscribe) = handle.split();

        S::allow_rw::<C, DRIVER_NUM, { allow_rw::ALLOW_GEN }>(allow_rw, buffer)?;

        S::subscribe::<_, _, C, DRIVER_NUM, 0>(subscribe, callback)?;

        S::command(DRIVER_NUM, command::GEN, num, 0).to_result::<(), ErrorCode>()
    }
}

// -----------------------------------------------------------------------------
// Driver number and command IDs
// -----------------------------------------------------------------------------

#[cfg(test)]
mod tests;
const DRIVER_NUM: u32 = 0x40001;

// Command IDs
#[allow(unused)]
mod command {
    pub const DRIVER_CHECK: u32 = 0;
    pub const GEN: u32 = 1;
}

#[allow(unused)]
mod subscribe {
    use libtock_platform::subscribe;
    pub const SUBSCRIBE_GEN: u32 = 0;
}

mod allow_rw {
    pub const ALLOW_GEN: u32 = 0;
}
