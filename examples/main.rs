#[macro_use] extern crate freenect;
extern crate libc;
extern crate piston_window;
extern crate image as im;
extern crate gfx_device_gl;

mod app;

use app::App;
use std::thread;
use std::sync::mpsc::{Sender, channel};
use freenect::ffi::*;
use freenect::context::{Context, ContextDefault, StatusCode};
use freenect::device::{Device, DeviceDefault, RGBArray};

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

    let mut buffer = [0; 640*480*3];

    let (mut sender, receiver) = channel::<&mut RGBArray> ();

    thread::spawn (|| {
        let mut app = App::new ();
        app.init (receiver);
    });

    dev.set_user_data (&mut sender);

    freenect_set_video_callback! (dev, fn callback (array : &mut RGBArray, timestamp: u32) {
        let sender = dev.get_user_data::<Sender<&mut RGBArray>> ();
        sender.send (array);
    });

    dev.set_tilt_degs (0 as f64);
    dev.set_led (FreenectLedOptions::GREEN);
    dev.set_video_mode (FreenectFrameMode::find_video_mode (FreenectResolution::MEDIUM, FreenectVideoFormat::RGB));
    dev.set_video_buffer (&mut buffer);

    dev.start_video ();

    while context.process_events () == StatusCode::Success {
    }
}
