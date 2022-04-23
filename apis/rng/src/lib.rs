#![no_std]

use core::cell::Cell;

use libtock_platform as platform;
use libtock_platform::allow_rw::AllowRw;
use libtock_platform::subscribe::Subscribe;
use platform::{
    share::{self},
    DefaultConfig, ErrorCode, Syscalls,
};

pub struct Rng<
    S: Syscalls,
    C: platform::allow_rw::Config + platform::subscribe::Config = DefaultConfig,
>(S, C);

impl<S: Syscalls, C: platform::allow_rw::Config + platform::subscribe::Config> Rng<S, C> {
    #[inline(always)]
    pub fn driver_check() -> bool {
        S::command(DRIVER_NUM, command::DRIVER_CHECK, 0, 0).is_success()
    }

    // writes num random bytes to buffer, works sync
    pub fn gen(buffer: &mut [u8], num: u32) -> Result<(), ErrorCode> {
        if (buffer.len() as u32) < num {
            return Err(ErrorCode::BadRVal);
        }

        let called = core::cell::Cell::new(Option::<(u32,)>::None);
        share::scope::<
            (
                AllowRw<_, DRIVER_NUM, { allow_rw::ALLOW_GEN }>,
                Subscribe<_, DRIVER_NUM, { subscribe::SUBSCRIBE_GEN }>,
            ),
            _,
            _,
        >(|handle| {
            let (allow_rw, subscribe) = handle.split();

            S::allow_rw::<C, DRIVER_NUM, { allow_rw::ALLOW_GEN }>(allow_rw, buffer)?;

            S::subscribe::<_, _, C, DRIVER_NUM, { subscribe::SUBSCRIBE_GEN }>(subscribe, &called)?;

            S::command(DRIVER_NUM, command::GEN, num, 0).to_result()?;

            S::yield_wait();
            if let Some((_,)) = called.get() {
                return Ok(());
            }
            Err(ErrorCode::Fail)
        })
    }

    pub fn gen_async<'share>(
        callback: &'share Cell<Option<(u32,)>>,
        buffer: &'share mut [u8],
        handle: share::Handle<(
            AllowRw<'share, S, DRIVER_NUM, 0>,
            Subscribe<'share, S, DRIVER_NUM, 0>,
        )>,
        num: u32,
    ) -> Result<(), ErrorCode> {
        let (allow_rw, subscribe) = handle.split();

        S::allow_rw::<C, DRIVER_NUM, { allow_rw::ALLOW_GEN }>(allow_rw, buffer)?;

        S::subscribe::<_, _, C, DRIVER_NUM, 0>(subscribe, callback)?;

        S::command(DRIVER_NUM, command::GEN, num, 0).to_result::<(), ErrorCode>()
    }

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
