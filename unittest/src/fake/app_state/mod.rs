use core::cell::Cell;
use libtock_platform::{CommandReturn, ErrorCode};
use crate::RoAllowBuffer;
use crate::upcall;
use core::ptr;

// ram_pointer: Box<T>
// flash pointer: Box<T>
// app_state_init is used once by each process

pub struct AppState {
    flash_ptr: Cell<u32>,
    header: Cell<usize>,
    ram_ptr: Cell<*mut u32>,
    flash_buffer: Cell<Vec<u8>>,
    buffer: Cell<RoAllowBuffer>,
    inited: Cell<bool>,
}
impl AppState{

    // pub fn init(&self){
    //     self.inited.set( true);   
    // }
    pub fn new() -> std::rc::Rc<AppState> {
        
        std::rc::Rc::new(AppState {
            flash_ptr: Default::default(),
            ram_ptr: Cell::new(ptr::null_mut()),
            header: Default::default(),
            flash_buffer: Default::default(),
            buffer: Default::default(),
            inited: Cell::new(false),
        })
    }
    /// Returns the bytes that have been submitted so far,
    /// and clears them.
    // pub fn init_ram_ptr(&self, ptr: *mut T) {
    //     self.ram_ptr = ptr;
    // }

    pub fn take_bytes(&self) -> Vec<u8> {
        self.flash_buffer.take()
    }
    // set header bits for 64-bit addresses
    pub fn set_header(&self, val: usize) {
        self.header.set(val);
    }
}

impl crate::fake::SyscallDriver for AppState { 
    fn id(&self) -> u32 {
        DRIVER_NUM
    }
    fn num_upcalls(&self) -> u32 {
        2
    }

    fn allow_readonly(
        &self,
        buffer_num: u32,
        buffer: RoAllowBuffer,
    ) -> Result<RoAllowBuffer, (RoAllowBuffer, ErrorCode)> {
        if buffer_num == SET_WRITE_BUFFER {
            Ok(self.buffer.replace(buffer))
        } else {
            Err((buffer, ErrorCode::Invalid))
        }
        
    }
    // write from ram(buffer) to flash_addr as arg0
    // for command we can just copy ram to flash
    fn command(&self, command_id: u32, argument0: u32, _argument1: u32) -> CommandReturn {
        match command_id {
            DRIVER_CHECK => {}
            WRITE_FLASH => {
                
                self.flash_ptr.set(argument0);     
                let flash_ptr = self.flash_ptr.as_ptr();
               
                let mut flash_buffer = vec![];    
                let buffer = self.buffer.take();
                let size = buffer.len();
                flash_buffer.extend_from_slice(&(*buffer)[..size]);
                
                unsafe{
                    ptr::copy((&mut flash_buffer).as_ptr(), flash_ptr as *mut u8, size);
                }

                self.buffer.set(buffer);
                self.flash_buffer.set(flash_buffer);
                
                upcall::schedule(DRIVER_NUM, SUBSCRIBE_WRITE, (size as u32, 0, 0))
                    .expect("Unable to schedule upcall {}");
            }
            _ => return crate::command_return::failure(ErrorCode::NoSupport),
        }
        crate::command_return::success()
    }
}


// static int app_state_init(void) {
//     allow_ro_return_t aret =
//       allow_readonly(DRIVER_NUM_APP_FLASH, 0, _app_state_ram_pointer, _app_state_size);
//     if (!aret.success) {
//       return tock_status_to_returncode(aret.status);
//     }
  
//     // Check that we have a region to use for this.
//     int number_regions = tock_app_number_writeable_flash_regions();
//     if (number_regions == 0) return RETURNCODE_ENOMEM;
  
//     // Get the pointer to flash which we need to ask the kernel where it is.
//     _app_state_flash_pointer = tock_app_writeable_flash_region_begins_at(0);
  
//     _app_state_inited = true;
//     return 0;
//   }

const DRIVER_NUM: u32 = 0x50000;
const FLASH_REGION_START: u32 = 0x11000;
// Command numbers
const DRIVER_CHECK: u32 = 0;
const WRITE_FLASH: u32 = 1;
//const READ: u32 = 2;
//const ABORT: u32 = 3;
const SUBSCRIBE_WRITE: u32 = 0;
// allow readonly numbers
const SET_WRITE_BUFFER: u32 = 0;
