/// SyscallLogEntry represents a system call made during test execution.
#[derive(Debug, PartialEq)]
pub enum SyscallLogEntry {
    // -------------------------------------------------------------------------
    // Yield
    // -------------------------------------------------------------------------
    YieldNoWait,

    YieldWait,

    // -------------------------------------------------------------------------
    // Subscribe
    // -------------------------------------------------------------------------
    Subscribe {
        driver_num: u32,
        subscribe_num: u32,
    },

    // -------------------------------------------------------------------------
    // Command
    // -------------------------------------------------------------------------
    Command {
        driver_id: u32,
        command_id: u32,
        argument0: u32,
        argument1: u32,
    },

    // -------------------------------------------------------------------------
    // Read-Only Allow
    // -------------------------------------------------------------------------
    AllowRo {
        driver_num: u32,
        buffer_num: u32,
        len: usize,
    },

    // -------------------------------------------------------------------------
    // Read-Write Allow
    // -------------------------------------------------------------------------
    AllowRw {
        driver_num: u32,
        buffer_num: u32,
        len: usize,
    },

    // -------------------------------------------------------------------------
    // Memop
    // -------------------------------------------------------------------------
    Memop {
        operation_id: u32,
        operation_arg: u32,
    },
    // TODO: Add Exit.
}
