use ffi::*;
use libc::c_void;
use utils::{str_to_const_c, catch_error_code};
use context::{Context, StatusCode};
use std::ops::Drop;
use std::mem::uninitialized;

pub struct Device {
    pub ptr : FreenectDevice,
}

impl Device {
    pub fn open_device (context : &Context, index : isize) -> Option<Device> {
        unsafe {
            let mut device = uninitialized ();
            let result = freenect_open_device (context.ptr, &mut device, index as i32);

            catch_error! (result;
            0 => Some (Device {
                ptr: device,
            }))
        }
    }

    pub fn open_device_by_camera_serial (context : &Context, camera_serial : &str) -> Option<Device> {
        unsafe {
            let mut device = uninitialized ();
            let result = freenect_open_device_by_camera_serial (context.ptr, &mut device, str_to_const_c (camera_serial));

            catch_error! (result;
            0 => Some (Device {
                ptr: device,
            }))
        }
    }

    pub fn set_user_data <'a, 'b, T> (&'a mut self, user_data : &'b mut T)
        where 'a : 'b {
        unsafe { freenect_set_user (self.ptr, user_data as *mut _ as *mut c_void); }
    }

    pub fn get_user_data <'a, 'b, T> (&'a self) ->  &'b mut T
        where 'a : 'b {
        unsafe { &mut *(freenect_get_user (self.ptr) as *mut T) }
    }

    pub fn start_video (&mut self) -> StatusCode {
        unsafe {
            let result = freenect_start_video (self.ptr);
            catch_error_code (result)
        }
    }

    pub fn start_depth (&mut self) -> StatusCode {
        unsafe {
            let result = freenect_start_depth (self.ptr);
            catch_error_code (result)
        }
    }

    pub fn stop_video (&mut self) -> StatusCode {
        unsafe {
            let result = freenect_stop_video (self.ptr);
            catch_error_code (result)
        }
    }

    pub fn stop_depth (&mut self) -> StatusCode {
        unsafe {
            let result = freenect_stop_depth (self.ptr);
            catch_error_code (result)
        }
    }

    pub fn update_tilt_state (&mut self) -> StatusCode {
        unsafe {
            let result = freenect_update_tilt_state (self.ptr);
            catch_error_code (result)
        }
    }

    pub fn get_tilt_state <'a, 'b> (&'a self) -> &'b mut FreenectRawTiltState
        where 'a : 'b {
        unsafe { &mut *(freenect_get_tilt_state (self.ptr)) }
    }

    pub fn get_tilt_degs (&self) -> f64 {
        self.get_tilt_state ().get_tilt_degs ()
    }

    pub fn set_tilt_degs (&mut self, degs : f64) -> StatusCode {
        unsafe {
            let result = freenect_set_tilt_degs (self.ptr, degs);
            catch_error_code (result)
        }
    }

    pub fn get_tilt_status (&self) -> FreenectTiltStatusCode {
        self.get_tilt_state ().get_tilt_status ()
    }

    pub fn set_led (&mut self, led : FreenectLedOptions) -> StatusCode {
        unsafe {
            let result = freenect_set_led (self.ptr, led);
            catch_error_code (result)
        }
    }

    pub fn get_current_video_mode (&self) -> FreenectFrameMode {
        unsafe { freenect_get_current_video_mode (self.ptr) }
    }

    pub fn set_video_mode (&mut self, mode : FreenectFrameMode) -> StatusCode {
        unsafe {
            let result = freenect_set_video_mode (self.ptr, mode);

            catch_error_code (result)
        }
    }

    pub fn set_video_buffer <'a, 'b> (&'a mut self, buffer : &'b mut [u8; 640*480*3]) -> StatusCode
        where 'a : 'b {
        unsafe {
            let result = freenect_set_video_buffer (self.ptr, buffer as *mut _ as *mut c_void);
            catch_error_code (result)
        }
    }
}

#[derive(Debug)]
#[repr(C, packed)]
pub struct RGBPacket {
    pub r : u8,
    pub g : u8,
    pub b : u8,
}

pub type RGBArray <'a> = &'a mut [RGBPacket; 640*480];

#[macro_export]
macro_rules! freenect_set_video_callback {
    ($device:ident, fn $cb_id:ident ($video_id:ident : RGBArray, $timestamp_id:ident : u32) $body:block) => {

        extern fn $cb_id (dev           : FreenectDevice,
                          $video_id     : *mut libc::c_void,
                          $timestamp_id : u32) {
            unsafe {
                let $video_id = &mut *($video_id as *mut [$crate::device::RGBPacket; 640*480]);
                $body
            }
        }


        unsafe { freenect_set_video_callback ($device.ptr, Some ($cb_id)); }
    };
}

/*#[macro_export]
macro_rules! freenect_set_depth_callback {
    ($device:ident, fn $cb_id:ident ($video_id:ident : RGBArray, $timestamp_id:ident : u32) $body:block) => {

        extern fn $cb_id (dev           : FreenectDevice,
                          $video_id     : *mut libc::c_void,
                          $timestamp_id : u32) {
            unsafe {
                let $video_id = &mut *($video_id as *mut [$crate::device::RGBPacket; 640*480]);
                $body
            }
        }


        unsafe { freenect_set_video_callback ($device.ptr, Some ($cb_id)); }
    };
}*/

impl Drop for Device {
    fn drop (&mut self) {
        unsafe { freenect_close_device (self.ptr) };
    }
}
