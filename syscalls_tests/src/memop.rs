use libtock_platform::memop_id;
use libtock_platform::Syscalls;
use libtock_unittest::{fake, ExpectedSyscall, SyscallLogEntry};

#[test]
fn memop_memory_begins_at() {
    let kernel = fake::Kernel::new();
    kernel.add_expected_syscall(ExpectedSyscall::Memop {
        operation_id: memop_id::MEMORY_START,
        operation_arg: 0,
        override_return: Some(dummy_addresses::MEM_START),
    });
    assert_eq!(
        fake::Syscalls::memop_memory_begins_at(),
        Ok(dummy_addresses::MEM_START)
    );
    assert_eq!(
        kernel.take_syscall_log(),
        [SyscallLogEntry::Memop {
            operation_id: 2,
            operation_arg: 0,
        }]
    );
}

#[test]
fn memop_memory_ends_at() {
    let kernel = fake::Kernel::new();
    kernel.add_expected_syscall(ExpectedSyscall::Memop {
        operation_id: memop_id::MEMORY_END,
        operation_arg: 0,
        override_return: Some(dummy_addresses::MEM_END),
    });
    assert_eq!(
        fake::Syscalls::memop_memory_ends_at(),
        Ok(dummy_addresses::MEM_END)
    );
    assert_eq!(
        kernel.take_syscall_log(),
        [SyscallLogEntry::Memop {
            operation_id: memop_id::MEMORY_END,
            operation_arg: 0,
        }]
    );
}

#[test]
fn memop_flash_starts_at() {
    let kernel = fake::Kernel::new();
    kernel.add_expected_syscall(ExpectedSyscall::Memop {
        operation_id: memop_id::FLASH_START,
        operation_arg: 0,
        override_return: Some(dummy_addresses::FLASH_START),
    });
    assert_eq!(
        fake::Syscalls::memop_flash_begins_at(),
        Ok(dummy_addresses::FLASH_START)
    );
    assert_eq!(
        kernel.take_syscall_log(),
        [SyscallLogEntry::Memop {
            operation_id: memop_id::FLASH_START,
            operation_arg: 0,
        }]
    );
}

#[test]
fn memop_flash_ends_at() {
    let kernel = fake::Kernel::new();
    kernel.add_expected_syscall(ExpectedSyscall::Memop {
        operation_id: memop_id::FLASH_END,
        operation_arg: 0,
        override_return: Some(dummy_addresses::FLASH_END),
    });
    assert_eq!(
        fake::Syscalls::memop_flash_ends_at(),
        Ok(dummy_addresses::FLASH_END)
    );
    assert_eq!(
        kernel.take_syscall_log(),
        [SyscallLogEntry::Memop {
            operation_id: memop_id::FLASH_END,
            operation_arg: 0,
        }]
    );
}

#[test]
fn memop_grant_begins_at() {
    let kernel = fake::Kernel::new();
    kernel.add_expected_syscall(ExpectedSyscall::Memop {
        operation_id: memop_id::GRANT_START,
        operation_arg: 0,
        override_return: Some(dummy_addresses::GRANT_START),
    });
    assert_eq!(
        fake::Syscalls::memop_grant_begins_at(),
        Ok(dummy_addresses::GRANT_START)
    );
    assert_eq!(
        kernel.take_syscall_log(),
        [SyscallLogEntry::Memop {
            operation_id: memop_id::GRANT_START,
            operation_arg: 0,
        }]
    );
}

#[test]
fn memop_number_writeable_flash_regions() {
    let kernel = fake::Kernel::new();
    kernel.add_expected_syscall(ExpectedSyscall::Memop {
        operation_id: memop_id::FLASH_REGIONS,
        operation_arg: 0,
        override_return: Some(NUM_WRITEABLE_REGIONS as *const u32),
    });
    assert_eq!(
        fake::Syscalls::memop_number_writeable_flash_regions(),
        Ok(NUM_WRITEABLE_REGIONS)
    );
    assert_eq!(
        kernel.take_syscall_log(),
        [SyscallLogEntry::Memop {
            operation_id: memop_id::FLASH_REGIONS,
            operation_arg: 0,
        }]
    );
}
#[test]
fn memop_flash_region_begins_at() {
    let region_index: u32 = 1;
    let kernel = fake::Kernel::new();
    kernel.add_expected_syscall(ExpectedSyscall::Memop {
        operation_id: memop_id::FLASH_REGIONS_START,
        operation_arg: region_index,
        override_return: Some(dummy_addresses::FLASH_REGION_START),
    });
    assert_eq!(
        fake::Syscalls::memop_flash_region_begins_at(region_index),
        Ok(dummy_addresses::FLASH_REGION_START)
    );
    assert_eq!(
        kernel.take_syscall_log(),
        [SyscallLogEntry::Memop {
            operation_id: memop_id::FLASH_REGIONS_START,
            operation_arg: region_index,
        }]
    );
}
#[test]
fn memop_flash_region_ends_at() {
    let region_index: u32 = 1;
    let kernel = fake::Kernel::new();
    kernel.add_expected_syscall(ExpectedSyscall::Memop {
        operation_id: memop_id::FLASH_REGIONS_END,
        operation_arg: region_index,
        override_return: Some(dummy_addresses::FLASH_REGION_END),
    });
    assert_eq!(
        fake::Syscalls::memop_flash_region_ends_at(region_index),
        Ok(dummy_addresses::FLASH_REGION_END)
    );
    assert_eq!(
        kernel.take_syscall_log(),
        [SyscallLogEntry::Memop {
            operation_id: memop_id::FLASH_REGIONS_END,
            operation_arg: region_index,
        }]
    );
}

const NUM_WRITEABLE_REGIONS: u32 = 2;
mod dummy_addresses {
    pub const MEM_START: *const u32 = 0x20000 as *const u32;
    pub const MEM_END: *const u32 = 0x22000 as *const u32;
    pub const FLASH_START: *const u32 = 0x10000 as *const u32;
    pub const FLASH_END: *const u32 = 0x14000 as *const u32;
    pub const GRANT_START: *const u32 = 0x21c00 as *const u32;
    pub const FLASH_REGION_START: *const u32 = 0x11000 as *const u32;
    pub const FLASH_REGION_END: *const u32 = 0x13000 as *const u32;
}
