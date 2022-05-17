// fake implementation of random number generator (RNG)
// was inspired from fake implementations of Console
use core::cell::Cell;
use core::cmp;
use libtock_platform::{CommandReturn, ErrorCode};

use crate::upcall;
use crate::RwAllowBuffer;

pub struct Rng {
    messages: Cell<Vec<u8>>,
    buffer: Cell<RwAllowBuffer>,
}

impl Rng {
    pub fn new() -> std::rc::Rc<Rng> {
        std::rc::Rc::new(Rng {
            messages: Default::default(),
            buffer: Default::default(),
        })
    }

    /// Returns the bytes that have been submitted so far,
    /// and clears them.
    pub fn take_bytes(&self) -> Vec<u8> {
        self.messages.take()
    }
}
// the fake implementation of rng driver
impl crate::fake::SyscallDriver for Rng {
    fn id(&self) -> u32 {
        DRIVER_NUM
    }
    // returns the max number of upcalls allowed,
    // more upcalls will result in a error from fake kernel
    fn num_upcalls(&self) -> u32 {
        2
    }
    // fake driver saves the buffer passed
    fn allow_readwrite(
        &self,
        buffer_num: u32,
        buffer: RwAllowBuffer,
    ) -> Result<RwAllowBuffer, (RwAllowBuffer, ErrorCode)> {
        if buffer_num == ALLOW_GEN {
            Ok(self.buffer.replace(buffer))
        } else {
            Err((buffer, ErrorCode::Invalid))
        }
    }

    // fake driver fills the buffer with 12s
    fn command(&self, command_num: u32, argument0: u32, _argument1: u32) -> CommandReturn {
        match command_num {
            DRIVER_CHECK => {}
            GEN => {
                let mut bytes = self.messages.take();
                let mut buffer = self.buffer.take();
                let size = cmp::min(buffer.len(), argument0 as usize);

                buffer.as_mut().fill(12);

                bytes.clear();
                bytes.extend_from_slice(&(*buffer)[..size]);

                self.buffer.set(buffer);
                self.messages.set(bytes);
                upcall::schedule(DRIVER_NUM, SUBSCRIBE_GEN, (size as u32, 0, 0))
                    .expect("Unable to schedule upcall {}");
            }
            _ => return crate::command_return::failure(ErrorCode::NoSupport),
        }
        crate::command_return::success()
    }
}

const DRIVER_NUM: u32 = 0x40001;
// Command numbers
const DRIVER_CHECK: u32 = 0;
const GEN: u32 = 1;
const SUBSCRIBE_GEN: u32 = 0;
const ALLOW_GEN: u32 = 0;
