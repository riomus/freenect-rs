#[macro_use] extern crate freenect;
extern crate libc;
extern crate piston_window;
extern crate image as im;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

mod app;

use im::{ImageBuffer, Rgba};
use app::App;
use std::thread;
use std::sync::{Mutex, Arc};
use freenect::ffi::*;
use freenect::context::{Context, ContextDefault, StatusCode};
use freenect::device::{Device, DeviceDefault};
use freenect::buffer::{RGBBufferVideoMedium, Buffer};

fn main () {
    let mut context = Context::init (None).unwrap ();

    context.set_log_level (FreenectLogLevel::DEBUG);
    freenect_set_log_callback! (context, fn cb (log_level : FreenectLogLevel, m : &str) {
        println! ("[LOG_LEVEL {:?}] {}", log_level, m);
    });
    context.select_subdevices(vec![FreenectDeviceFlags::CAMERA]);

    let n = context.num_devices ().unwrap ();
    println! ("Number of devices: {}", n);

    let mut dev = Device::open_device (&context, 0).unwrap ();
    let mut devbuffer : RGBBufferVideoMedium = Buffer::new ();
    let canvas  = Arc::new (Mutex::new (ImageBuffer::from_pixel (640, 480, Rgba([255; 4]))));

    dev.set_user_data (&mut canvas.clone ());

    freenect_set_video_callback! (dev, fn callback (array : &mut RGBBufferVideoMedium, timestamp: u32) {
        let canvas : &mut Arc<Mutex<ImageBuffer<Rgba<u8> ,Vec<u8>>>> = dev.get_user_data ();
        let mut canvas = canvas.lock ().unwrap ();
        let mut j = 0;
        for (data, pixel) in array.iter ().zip (canvas.pixels_mut ()) {
            *pixel = Rgba ([data.r, data.g, data.b, 255]);
        }
    });

    dev.set_tilt_degs (0 as f64);
    dev.set_led (FreenectLedOptions::GREEN);
    dev.set_video_mode (FreenectFrameMode::find_video_mode (FreenectResolution::MEDIUM, FreenectVideoFormat::RGB));
    dev.set_video_buffer (&mut devbuffer);

    dev.start_video ();

    let new_ref = canvas.clone ();

    thread::spawn (|| {
        let mut app = App::new ();
        app.init (new_ref);
    });

    while context.process_events () == StatusCode::Success {
    }
}
