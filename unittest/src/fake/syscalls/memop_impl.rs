use crate::kernel_data::with_kernel_data;
use crate::{ExpectedSyscall, SyscallLogEntry};
use core::ptr;
use libtock_platform::{return_variant, Register};

// fake memop system call
pub(super) fn memop(op_id: Register, op_arg: Register) -> [Register; 2] {
    let operation_id = op_id.try_into().expect("Too large operation ID");
    let operation_arg = op_arg.try_into().expect("Too large operation argument");

    let override_return = with_kernel_data(|option_kernel_data| {
        let kernel_data = option_kernel_data.expect("Memop called but no fake::Kernel exists");

        kernel_data.syscall_log.push(SyscallLogEntry::Memop {
            operation_id,
            operation_arg,
        });

        // require that expected system call was last?
        // check for syscall entry
        let override_return = match kernel_data.expected_syscalls.pop_front() {
            None => None,
            Some(ExpectedSyscall::Memop {
                operation_id: expected_operation_id,
                operation_arg: expected_operation_arg,
                override_return,
            }) => {
                assert_eq!(
                    operation_id, expected_operation_id,
                    "expected different operation_id"
                );
                assert_eq!(
                    operation_arg, expected_operation_arg,
                    "expected different operation_arg"
                );
                override_return
            }
            Some(expected_syscall) => expected_syscall.panic_wrong_call("Memop"),
        };

        override_return
    });
    // the fake memop implementation was only used to test app_state driver
    let return_variant: u32 = return_variant::SUCCESS_U32.into();
    let res: *const u32 = match override_return {
        Some(val) => val,
        None => match operation_id {
            memop_id::MEMORY_START => dummy_addresses::MEM_START,
            memop_id::MEMORY_END => dummy_addresses::MEM_END,
            memop_id::FLASH_START => dummy_addresses::FLASH_START,
            memop_id::FLASH_END => dummy_addresses::FLASH_END,
            memop_id::GRANT_START => dummy_addresses::GRANT_START,
            memop_id::FLASH_REGIONS => {
                let n = FLASH_REGION_NUM;
                return [return_variant.into(), n.into()];
            }
            memop_id::FLASH_REGIONS_START => {
                unsafe {
                    let flash_addr = &mut FLASH_BUFFER as *mut u8 as *mut u32;
                    // assert_eq!((flash_addr as u32) as usize, flash_addr as usize);
                    // assertion works for usize
                    assert_eq!(ptr::read(flash_addr as usize as *const u32), 0);
                    flash_addr
                }
            }
            memop_id::FLASH_REGIONS_END => dummy_addresses::FLASH_REGION_END,
            _ => panic!("Operation id not in correct range"),
        },
    };

    [return_variant.into(), res.into()]
}

static mut FLASH_BUFFER: [u8; 20] = [0u8; 20];
const FLASH_REGION_NUM: usize = 1;
mod memop_id {
    pub const MEMORY_START: u32 = 2;
    pub const MEMORY_END: u32 = 3;
    pub const FLASH_START: u32 = 4;
    pub const FLASH_END: u32 = 5;
    pub const GRANT_START: u32 = 6;
    pub const FLASH_REGIONS: u32 = 7;
    pub const FLASH_REGIONS_START: u32 = 8;
    pub const FLASH_REGIONS_END: u32 = 9;
}

mod dummy_addresses {
    pub const MEM_START: *const u32 = 0x20000 as *const u32;
    pub const MEM_END: *const u32 = 0x22000 as *const u32;
    pub const FLASH_START: *const u32 = 0x10000 as *const u32;
    pub const FLASH_END: *const u32 = 0x14000 as *const u32;
    pub const GRANT_START: *const u32 = 0x21c00 as *const u32;
    pub const FLASH_REGION_END: *const u32 = 0x13000 as *const u32;
}
