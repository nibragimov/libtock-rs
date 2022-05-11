use super::*;
use libtock_platform::ErrorCode;
use libtock_unittest::{command_return, fake, ExpectedSyscall};

type Rng = super::Rng<fake::Syscalls>;

#[test]
fn no_driver() {
    let _kernel = fake::Kernel::new();
    assert!(!Rng::driver_check());
}

#[test]
fn driver_check() {
    let kernel = fake::Kernel::new();
    let driver = fake::Rng::new();
    kernel.add_driver(&driver);

    assert!(Rng::driver_check());
    assert_eq!(driver.take_bytes(), []);
}

// we only check buffer write
// randomness is checked in capsule tests
#[test]
fn gen_bytes() {
    let kernel = fake::Kernel::new();
    let driver = fake::Rng::new();
    kernel.add_driver(&driver);

    let mut buffer = [0u8; 4];
    let num = buffer.len() as u32;
    Rng::gen(&mut buffer, num).unwrap();
    assert_eq!(driver.take_bytes(), [12, 12, 12, 12]);
}
#[test]
fn gen_bytes_async() {
    let kernel = fake::Kernel::new();
    let driver = fake::Rng::new();
    kernel.add_driver(&driver);

    let mut buffer = [0u8; 4];
    let num = buffer.len() as u32;

    let callback = Cell::new(Option::<(u32,)>::None);
    let _ret: Result<(), ErrorCode> = share::scope(|handle| {
        Rng::gen_async(&callback, &mut buffer, handle, num)?;
        _yield(&callback)?;
        Ok(())
    });

    assert_eq!(driver.take_bytes(), [12, 12, 12, 12]);
}

#[test]
fn gen_bytes_async_mult() {
    let kernel = fake::Kernel::new();
    let driver = fake::Rng::new();
    kernel.add_driver(&driver);

    let mut buffer = [0u8; 4];
    let mut buffer2 = [1u8; 3];
    let num = buffer.len() as u32;
    let num2 = buffer2.len() as u32;

    let callback = Cell::new(Option::<(u32,)>::None);
    let callback2 = Cell::new(Option::<(u32,)>::None);
    let _ret: Result<(), ErrorCode> = share::scope(|handle| {
        Rng::gen_async(&callback, &mut buffer, handle, num)?;
        Rng::gen_async(&callback2, &mut buffer2, handle, num2)?;
        _yield(&callback)?;
        _yield(&callback2)?;

        Ok(())
    });
    assert_eq!(driver.take_bytes(), [12, 12, 12]);
    assert_eq!(buffer2, [12, 12, 12]);
    assert_eq!(buffer, [12, 12, 12, 12]);
}
#[test]
fn failed_gen() {
    let kernel = fake::Kernel::new();
    let driver = fake::Rng::new();
    let mut buffer = [0u8; 4];
    let size = buffer.len();
    kernel.add_driver(&driver);

    kernel.add_expected_syscall(ExpectedSyscall::AllowRw {
        driver_num: DRIVER_NUM,
        buffer_num: subscribe::SUBSCRIBE_GEN,
        return_error: None,
    });
    kernel.add_expected_syscall(ExpectedSyscall::Subscribe {
        driver_num: DRIVER_NUM,
        subscribe_num: subscribe::SUBSCRIBE_GEN,
        skip_with_error: None,
    });
    kernel.add_expected_syscall(ExpectedSyscall::Command {
        driver_id: DRIVER_NUM,
        command_id: command::GEN,
        argument0: buffer.len() as u32,
        argument1: 0,
        override_return: Some(command_return::failure(ErrorCode::Fail)),
    });

    assert_eq!(Rng::gen(&mut buffer, size as u32), Err(ErrorCode::Fail));
    // The fake driver still receives the command even if a fake error is injected.
    assert_eq!(driver.take_bytes(), [12, 12, 12, 12]);
}

#[test]
fn small_buffer() {
    let kernel = fake::Kernel::new();
    let driver = fake::Rng::new();
    let mut buffer = [0u8; 4];
    kernel.add_driver(&driver);
    //let reduced_len = 3;
    assert_eq!(Rng::gen(&mut buffer, 5u32), Err(ErrorCode::BadRVal));
}

// helper function to wait for output
fn _yield(callback: &Cell<Option<(u32,)>>) -> Result<(), ErrorCode> {
    fake::Syscalls::yield_wait();
    match (*callback).get() {
        Some((_,)) => Ok(()),
        _ => Err(ErrorCode::Fail),
    }
}

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
