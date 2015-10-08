use libc::c_void;

#[derive(Debug, Clone, Copy, Default)]
#[repr(C, packed)]
pub struct RGBPacket {
    pub r : u8,
    pub g : u8,
    pub b : u8,
}

pub type RGBBufferVideoMedium = [RGBPacket; 640*480];

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
