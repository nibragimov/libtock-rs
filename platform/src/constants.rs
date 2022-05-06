//! Defines constants shared between multiple `libtock-rs` crates.

pub mod exit_id {
    pub const TERMINATE: u32 = 0;
    pub const RESTART: u32 = 1;
}

pub mod syscall_class {
    pub const SUBSCRIBE: usize = 1;
    pub const COMMAND: usize = 2;
    pub const ALLOW_RW: usize = 3;
    pub const ALLOW_RO: usize = 4;
    pub const MEMOP: usize = 5;
    pub const EXIT: usize = 6;
}

pub mod yield_id {
    pub const NO_WAIT: u32 = 0;
    pub const WAIT: u32 = 1;
}

pub mod memop_id {
    pub const BRK: u32 = 0;
    pub const SBRK: u32 = 1;
    pub const MEMORY_START: u32 = 2;
    pub const MEMORY_END: u32 = 3;
    pub const FLASH_START: u32 = 4;
    pub const FLASH_END: u32 = 5;
    pub const GRANT_START: u32 = 6;
    pub const FLASH_REGIONS: u32 = 7;
    pub const FLASH_REGIONS_START: u32 = 8;
    pub const FLASH_REGIONS_END: u32 = 9;
    pub const STACK_LOCATION: u32 = 10;
    pub const HEAP_LOCATION: u32 = 11;
}
