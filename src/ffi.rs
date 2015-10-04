#![allow(dead_code)]

use libc::{uint8_t, uint16_t, uint32_t, int8_t, int16_t, int32_t, c_int, c_uint, c_void, c_float,
    c_double, c_char, c_uchar};
use std::clone::Clone;
use std::default::Default;
use std::mem::{zeroed, transmute};

pub const DUMMY_VALUE : isize = 2147483647;

pub const FREENECT_COUNTS_PER_G : usize = 819;
pub const FREENECT_DEPTH_MM_MAX_VALUE : usize = 10000;
pub const FREENECT_DEPTH_MM_NO_VALUE : usize = 0;
pub const FREENECT_DEPTH_RAW_MAX_VALUE : usize = 2048;
pub const FREENECT_DEPTH_RAW_NO_VALUE : usize = 2047;

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum FreenectDeviceFlags {
    MOTOR  = 1 << 0,
    CAMERA = 1 << 1,
    AUDIO  = 1 << 2,
}

#[repr(C)]
#[derive(Copy)]
pub struct FreenectDeviceAttributes {
    pub next: *mut FreenectDeviceAttributes,
    pub camera_serial: *const c_char,
}
impl Clone for FreenectDeviceAttributes {
    fn clone(&self) -> Self { *self }
}
impl Default for FreenectDeviceAttributes {
    fn default() -> Self { unsafe { zeroed() } }
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub enum FreenectResolution {
    LOW    = 0,
    MEDIUM = 1,
    HIGH   = 2,
    DUMMY  = DUMMY_VALUE,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub enum FreenectVideoFormat {
    RGB             = 0,
    BAYER           = 1,
    IR_8BIT         = 2,
    IR_10BIT        = 3,
    IR_10BIT_PACKED = 4,
    YUV_RGB         = 5,
    YUV_RAW         = 6,
    DUMMY           = DUMMY_VALUE,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum FreenectDepthFormat {
    F_11BIT        = 0,
    F_10BIT        = 1,
    F_11BIT_PACKED = 2,
    F_10BIT_PACKED = 3,
    REGISTERED     = 4,
    MM             = 5,
    DUMMY          = DUMMY_VALUE,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum FreenectFlag {
    AUTO_EXPOSURE      = 1 << 14,
    AUTO_WHITE_BALANCE = 1 << 1,
    RAW_COLOR          = 1 << 4,
    MIRROR_DEPTH       = 1 << 16,
    MIRROR_VIDEO       = 1 << 17,
    NEAR_MODE          = 1 << 18,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum FreenectFlagValue {
    OFF = 0,
    ON  = 1,
}

#[repr(C)]
#[derive(Copy, Debug)]
pub struct FreenectFrameMode {
    pub reserved: uint32_t,
    pub resolution: FreenectResolution,
    pub _bindgen_data_1_: [u32; 1usize],
    pub bytes: int32_t,
    pub width: int16_t,
    pub height: int16_t,
    pub data_bits_per_pixel: int8_t,
    pub padding_bits_per_pixel: int8_t,
    pub framerate: int8_t,
    pub is_valid: int8_t,
}
impl FreenectFrameMode {
    pub unsafe fn dummy(&mut self) -> *mut int32_t {
        let raw: *mut u8 = transmute(&self._bindgen_data_1_);
        transmute(raw.offset(0))
    }
    pub unsafe fn video_format(&mut self) -> *mut FreenectVideoFormat {
        let raw: *mut u8 = transmute(&self._bindgen_data_1_);
        transmute(raw.offset(0))
    }
    pub unsafe fn depth_format(&mut self) -> *mut FreenectDepthFormat {
        let raw: *mut u8 = transmute(&self._bindgen_data_1_);
        transmute(raw.offset(0))
    }
    pub fn find_video_mode (res : FreenectResolution, fmt : FreenectVideoFormat) -> FreenectFrameMode {
        unsafe { freenect_find_video_mode (res, fmt) }
    }
    pub fn get_video_mode (n : isize) -> FreenectFrameMode {
        unsafe { freenect_get_video_mode (n as i32) }
    }
}
impl Clone for FreenectFrameMode {
    fn clone(&self) -> Self { *self }
}
impl Default for FreenectFrameMode {
    fn default() -> Self { unsafe { zeroed() } }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum FreenectLedOptions {
    OFF              = 0,
    GREEN            = 1,
    RED              = 2,
    YELLOW           = 3,
    BLINK_GREEN      = 4,
    BLINK_RED_YELLOW = 6,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub enum FreenectTiltStatusCode {
    STOPPED = 0,
    LIMIT   = 1 << 0,
    MOVING  = 1 << 2,
}

#[repr(C)]
#[derive(Copy)]
pub struct FreenectRawTiltState {
    pub accelerometer_x: int16_t,
    pub accelerometer_y: int16_t,
    pub accelerometer_z: int16_t,
    pub tilt_angle: int8_t,
    pub tilt_status: FreenectTiltStatusCode,
}
impl FreenectRawTiltState {
    pub fn get_tilt_degs (&mut self) -> f64 {
        unsafe { freenect_get_tilt_degs (self) }
    }

    pub fn get_tilt_status (&mut self) -> FreenectTiltStatusCode {
        unsafe {
            let result = freenect_get_tilt_status (self);
            result
        }
    }
}
impl Clone for FreenectRawTiltState {
    fn clone(&self) -> Self { *self }
}
impl Default for FreenectRawTiltState {
    fn default() -> Self { unsafe { zeroed() } }
}

pub enum StructFreenectContext {}
pub type FreenectContext = *mut StructFreenectContext;

pub enum StructFreenectDevice {}
pub type FreenectDevice = *mut StructFreenectDevice;

pub type FreenectUSBContext = *mut c_void;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub enum FreenectLogLevel {
    FATAL = 0,
    ERROR,
    WARNING,
    NOTICE,
    INFO,
    DEBUG,
    SPEW,
    FLOOD,
}

#[repr(C)]
#[derive(Copy)]
pub struct FreenectRegInfo {
    pub dx_center: int32_t,
    pub ax: int32_t,
    pub bx: int32_t,
    pub cx: int32_t,
    pub dx: int32_t,
    pub dx_start: int32_t,
    pub ay: int32_t,
    pub by: int32_t,
    pub cy: int32_t,
    pub dy: int32_t,
    pub dy_start: int32_t,
    pub dx_beta_start: int32_t,
    pub dy_beta_start: int32_t,
    pub rollout_blank: int32_t,
    pub rollout_size: int32_t,
    pub dx_beta_inc: int32_t,
    pub dy_beta_inc: int32_t,
    pub dxdx_start: int32_t,
    pub dxdy_start: int32_t,
    pub dydx_start: int32_t,
    pub dydy_start: int32_t,
    pub dxdxdx_start: int32_t,
    pub dydxdx_start: int32_t,
    pub dxdxdy_start: int32_t,
    pub dydxdy_start: int32_t,
    pub back_comp1: int32_t,
    pub dydydx_start: int32_t,
    pub back_comp2: int32_t,
    pub dydydy_start: int32_t,
}
impl Clone for FreenectRegInfo {
    fn clone(&self) -> Self { *self }
}
impl Default for FreenectRegInfo {
    fn default() -> Self { unsafe { zeroed() } }
}

#[repr(C)]
#[derive(Copy)]
pub struct FreenectRegPadInfo {
    pub start_lines: uint16_t,
    pub end_lines: uint16_t,
    pub cropping_lines: uint16_t,
}
impl Clone for FreenectRegPadInfo {
    fn clone(&self) -> Self { *self }
}
impl Default for FreenectRegPadInfo {
    fn default() -> Self { unsafe { zeroed() } }
}

#[repr(C)]
#[derive(Copy)]
pub struct FreenectZeroPlaneInfo {
    pub dcmos_emitter_dist: c_float,
    pub dcmos_rcmos_dist: c_float,
    pub reference_distance: c_float,
    pub reference_pixel_size: c_float,
}
impl Clone for FreenectZeroPlaneInfo {
    fn clone(&self) -> Self { *self }
}
impl Default for FreenectZeroPlaneInfo {
    fn default() -> Self { unsafe { zeroed() } }
}

#[repr(C)]
#[derive(Copy)]
pub struct FreenectRegistration {
    pub reg_info: FreenectRegInfo,
    pub reg_pad_info: FreenectRegPadInfo,
    pub zero_plane_info: FreenectZeroPlaneInfo,
    pub const_shift: c_double,
    pub raw_to_mm_shift: *mut uint16_t,
    pub depth_to_rgb_shift: *mut int32_t,
    pub registration_table: *mut c_void,
}
impl Clone for FreenectRegistration {
    fn clone(&self) -> Self { *self }
}
impl Default for FreenectRegistration {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}

#[repr(C)]
#[derive(Copy)]
pub struct FreenectSample51 {
    pub left: int16_t,
    pub right: int16_t,
    pub center: int16_t,
    pub lfe: int16_t,
    pub surround_left: int16_t,
    pub surround_right: int16_t,
}
impl Clone for FreenectSample51 {
    fn clone(&self) -> Self { *self }
}
impl Default for FreenectSample51 {
    fn default() -> Self { unsafe { zeroed() } }
}

pub type FreenectLogCb = Option<extern fn(dev: FreenectContext,
                                          level: FreenectLogLevel,
                                          msg: *const c_char)>;
pub type FreenectDepthCb = Option<extern fn(dev: FreenectDevice,
                                            depth: *mut c_void,
                                            timestamp: uint32_t)>;
pub type FreenectVideoCb = Option<extern fn(dev: FreenectDevice,
                                            video: *mut c_void,
                                            timestamp: uint32_t)>;
pub type FreenectChunkCb = Option<extern fn(buffer: *mut c_void,
                                            pkt_data: *mut c_void,
                                            pkt_num: c_int,
                                            datalen: c_int,
                                            user_data: *mut c_void)>;

pub type FreenectAudioInCb = Option<extern fn(dev: FreenectDevice,
                                              num_samples: c_int,
                                              mic1: *mut int32_t,
                                              mic2: *mut int32_t,
                                              mic3: *mut int32_t,
                                              mic4: *mut int32_t,
                                              cancelled: *mut int16_t,
                                              unknown: *mut c_void)>;

pub type FreenectAudioOutCb = Option<extern fn(dev: FreenectDevice,
                                               samples: *mut FreenectSample51,
                                               sample_count: *mut c_int)>;

#[link(name = "freenect")]
extern {
    pub fn freenect_init(ctx: *mut FreenectContext,
                         usb_ctx: FreenectUSBContext) -> c_int;
    pub fn freenect_shutdown(ctx: FreenectContext) -> c_int;
    pub fn freenect_set_log_level(ctx: FreenectContext,
                                  level: FreenectLogLevel);
    pub fn freenect_set_log_callback(ctx: FreenectContext,
                                     cb: FreenectLogCb);
    pub fn freenect_process_events(ctx: FreenectContext) -> c_int;
    /*pub fn freenect_process_events_timeout(ctx: FreenectContext,
                                           timeout: *mut Struct_timeval) -> c_int;*/
    pub fn freenect_num_devices(ctx: FreenectContext) -> c_int;
    pub fn freenect_list_device_attributes(ctx: FreenectContext,
                                           attribute_list: *mut *mut FreenectDeviceAttributes) -> c_int;
    pub fn freenect_free_device_attributes(attribute_list: *mut FreenectDeviceAttributes);
    pub fn freenect_supported_subdevices() -> c_int;
    pub fn freenect_select_subdevices(ctx: FreenectContext,
                                      subdevs: c_int);
    pub fn freenect_enabled_subdevices(ctx: FreenectContext) -> FreenectDeviceFlags;
    pub fn freenect_open_device(ctx: FreenectContext,
                                dev: *mut FreenectDevice,
                                index: c_int) -> c_int;
    pub fn freenect_open_device_by_camera_serial(ctx: FreenectContext,
                                                 dev: *mut FreenectDevice,
                                                 camera_serial: *const c_char) -> c_int;
    pub fn freenect_close_device(dev: FreenectDevice) -> c_int;
    pub fn freenect_set_user(dev: FreenectDevice,
                             user: *mut c_void);
    pub fn freenect_get_user(dev: FreenectDevice) -> *mut c_void;
    pub fn freenect_set_depth_callback(dev: FreenectDevice,
                                       cb: FreenectDepthCb);
    pub fn freenect_set_video_callback(dev: FreenectDevice,
                                       cb: FreenectVideoCb);
    pub fn freenect_set_depth_chunk_callback(dev: FreenectDevice,
                                             cb: FreenectChunkCb);
    pub fn freenect_set_video_chunk_callback(dev: FreenectDevice,
                                             cb: FreenectChunkCb);
    pub fn freenect_set_depth_buffer(dev: FreenectDevice,
                                     buf: *mut c_void) -> c_int;
    pub fn freenect_set_video_buffer(dev: FreenectDevice,
                                     buf: *mut c_void) -> c_int;
    pub fn freenect_start_depth(dev: FreenectDevice) -> c_int;
    pub fn freenect_start_video(dev: FreenectDevice) -> c_int;
    pub fn freenect_stop_depth(dev: FreenectDevice) -> c_int;
    pub fn freenect_stop_video(dev: FreenectDevice) -> c_int;
    pub fn freenect_update_tilt_state(dev: FreenectDevice) -> c_int;
    pub fn freenect_get_tilt_state(dev: FreenectDevice) -> *mut FreenectRawTiltState;
    pub fn freenect_get_tilt_degs(state: *mut FreenectRawTiltState) -> c_double;
    pub fn freenect_set_tilt_degs(dev: FreenectDevice,
                                  angle: c_double) -> c_int;
    pub fn freenect_get_tilt_status(state: *mut FreenectRawTiltState) -> FreenectTiltStatusCode;
    pub fn freenect_set_led(dev: FreenectDevice,
                            option: FreenectLedOptions) -> c_int;
    pub fn freenect_get_mks_accel(state: *mut FreenectRawTiltState,
                                  x: *mut c_double,
                                  y: *mut c_double,
                                  z: *mut c_double);
    pub fn freenect_get_video_mode_count() -> c_int;
    pub fn freenect_get_video_mode(mode_num: c_int) -> FreenectFrameMode;
    pub fn freenect_get_current_video_mode(dev: FreenectDevice) -> FreenectFrameMode;
    pub fn freenect_find_video_mode(res: FreenectResolution,
                                    fmt: FreenectVideoFormat) -> FreenectFrameMode;
    pub fn freenect_set_video_mode(dev: FreenectDevice,
                                   mode: FreenectFrameMode) -> c_int;
    pub fn freenect_get_depth_mode_count() -> c_int;
    pub fn freenect_get_depth_mode(mode_num: c_int) -> FreenectFrameMode;
    pub fn freenect_get_current_depth_mode(dev: FreenectDevice) -> FreenectFrameMode;
    pub fn freenect_find_depth_mode(res: FreenectResolution,
                                    fmt: FreenectDepthFormat) -> FreenectFrameMode;
    pub fn freenect_set_depth_mode(dev: FreenectDevice,
                                   mode: FreenectFrameMode) -> c_int;
    pub fn freenect_set_flag(dev: FreenectDevice, flag: FreenectFlag,
                             value: FreenectFlagValue) -> c_int;
    pub fn freenect_get_ir_brightness(dev: FreenectDevice) -> c_int;
    pub fn freenect_set_ir_brightness(dev: FreenectDevice,
                                      brightness: uint16_t) -> c_int;
    pub fn freenect_set_fw_address_nui(ctx: FreenectContext,
                                       fw_ptr: *mut c_uchar,
                                       num_bytes: c_uint);
    pub fn freenect_set_fw_address_k4w(ctx: FreenectContext,
                                       fw_ptr: *mut c_uchar,
                                       num_bytes: c_uint) ;
    pub fn freenect_set_audio_in_callback(dev: FreenectDevice,
                                          callback: FreenectAudioInCb);
    pub fn freenect_set_audio_out_callback(dev: FreenectDevice,
                                           callback: FreenectAudioOutCb);
    pub fn freenect_start_audio(dev: FreenectDevice) -> c_int;
    pub fn freenect_stop_audio(dev: FreenectDevice) -> c_int;
    pub fn freenect_copy_registration(dev: FreenectDevice) -> FreenectRegistration;
    pub fn freenect_destroy_registration(reg: *mut FreenectRegistration) -> c_int;
    pub fn freenect_camera_to_world(dev: FreenectDevice,
                                    cx: c_int,
                                    cy: c_int,
                                    wz: c_int,
                                    wx: *mut c_double,
                                    wy: *mut c_double);
    pub fn freenect_map_rgb_to_depth(dev: FreenectDevice,
                                     depth_mm: *mut uint16_t,
                                     rgb_raw: *mut uint8_t,
                                     rgb_registered: *mut uint8_t);
}
