use super::*;
use libtock_platform::ErrorCode;
use libtock_unittest::fake;

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
        Rng::_yield(&callback)?;
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
        Rng::_yield(&callback)?;
        Rng::_yield(&callback2)?;
        Ok(())
    });
    assert_eq!(driver.take_bytes(), [12, 12, 12]);
    assert_eq!(buffer2, [12, 12, 12]);
    assert_eq!(buffer, [12, 12, 12, 12]);
}
