use libc::c_void;
use std::convert::From;
#[derive(Debug, Clone, Copy, Default)]
#[repr(C, packed)]
pub struct RGBPacket {
    pub r : u8,
    pub g : u8,
    pub b : u8,
}

#[derive(Debug, Clone, Copy, Default)]
#[repr(C, packed)]
pub struct DepthPacket {
    pub d : u16
}

pub type RGBBufferVideoMedium = [RGBPacket; 640*480];
pub type DepthBufferVideoMedium = [DepthPacket; 640*480];

pub trait Buffer {
    fn new () -> Self;

    #[inline]
    fn to_unsafe (&mut self) -> *mut c_void {
        self as *mut _ as *mut c_void
    }
}

impl Buffer for RGBBufferVideoMedium {
    fn new () -> Self {
        [RGBPacket::default (); 640*480]
    }
}

impl Buffer for DepthBufferVideoMedium {
    fn new () -> Self {
        [DepthPacket::default (); 640*480]
    }
}

pub struct DepthBufferVideoMediumWrap(&'static mut DepthBufferVideoMedium);

impl From<*mut c_void> for DepthBufferVideoMediumWrap{
    fn from(data: *mut c_void) -> DepthBufferVideoMediumWrap{
        unsafe{
        let out=&mut *(data as *mut DepthBufferVideoMedium);
        DepthBufferVideoMediumWrap(out)
        }
    }
}

