use ffi::*;
use utils::catch_error_code;
use attributes::DeviceAttributes;
use std::ops::Drop;
use std::ptr::null_mut;
use std::mem::uninitialized;

pub enum StatusCode {
    Success,
    Failure,
}

pub struct Context {
    pub ptr : FreenectContext,
}

pub struct USBContext {
    pub ptr : FreenectUSBContext,
}

impl Context {
    pub fn init (usb_ctx : Option<USBContext>) -> Option<Context> {
        unsafe {
            let mut ptr = uninitialized ();
            let usb_ctx_pointer = match usb_ctx {
                None => null_mut (),
                Some (_) => null_mut (),
            };

            let result = freenect_init (&mut ptr, usb_ctx_pointer);

            catch_error! (result;
                0 => Some (Context { ptr : ptr }) )
        }
    }

    pub fn process_events (&mut self) -> StatusCode {
        unsafe {
            let result = freenect_process_events (self.ptr);
            catch_error_code (result)
        }
    }

    pub fn num_devices (&self) -> Option<usize> {
        unsafe {
            let result = freenect_num_devices (self.ptr);

            catch_error! (result;
                x @ _ if x >= 0 => Some (x as usize))
        }
    }

    pub fn list_device_attributes (&self) -> Option<Vec<DeviceAttributes>> {
        unsafe {
            let mut list = uninitialized ();
            let result = freenect_list_device_attributes (self.ptr, &mut list);

            catch_error! (result;
                x @ _ if x >= 0 =>
                {
                    let atts = DeviceAttributes::from_linked_list (list, x as usize);

                    Some (atts)
                })
        }
    }

    pub fn enabled_subdevices (&self) -> Vec<FreenectDeviceFlags> {
        unsafe {
            let result = freenect_enabled_subdevices (self.ptr) as u8;

            let mut vec = Vec::new ();
            switch_freenect_device_flags! (result;
                {vec.push (FreenectDeviceFlags::MOTOR);},
                {vec.push (FreenectDeviceFlags::CAMERA);},
                {vec.push (FreenectDeviceFlags::AUDIO);});

            vec
        }
    }

    pub fn select_subdevices (&mut self, flags : Vec<FreenectDeviceFlags>) {
        let mut total_flag = 0;

        for key in flags {
            total_flag = total_flag | key as u8;
        }

        unsafe {
            freenect_select_subdevices (self.ptr, total_flag as i32);
        }
    }

    pub fn set_log_level (&mut self, log_level : FreenectLogLevel) {
        unsafe { freenect_set_log_level (self.ptr, log_level); }
    }
}

#[macro_export]
macro_rules! freenect_set_log_callback {
    ($context:ident, fn $cb_id:ident ($log_level_id:ident : FreenectLogLevel, $str_id:ident : &str) $body:block) => {

        extern fn $cb_id (dev                          : FreenectContext,
                          $log_level_id                : FreenectLogLevel,
                          $str_id                      : *const libc::c_char) {
            unsafe {
                let $str_id = $crate::utils::const_c_to_string ($str_id);
                $body
            }
        }


        unsafe { freenect_set_log_callback ($context.ptr, Some ($cb_id)); }
    };
}


impl Drop for Context {
    fn drop (&mut self) {
        unsafe { freenect_shutdown (self.ptr) };
    }
}