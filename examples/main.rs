#[macro_use]
extern crate freenect;
extern crate libc;
extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;
extern crate image;

use freenect::ffi::*;
use freenect::context::{Context, StatusCode};
use freenect::device::{Device, RGBArray};
use opengl_graphics::{GlGraphics, OpenGL};
use piston::window::WindowSettings;
use glutin_window::GlutinWindow;
use piston::event_loop::*;
use piston::input::*;

mod paint;

fn main () {

    let mut context = Context::init (None).unwrap ();

    context.set_log_level (FreenectLogLevel::DEBUG);

    freenect_set_log_callback! (context, fn cb (log_level : FreenectLogLevel, m : &str) {
        println! ("[LOG_LEVEL {:?}] {}", log_level, m);
    });

    context.select_subdevices(vec![FreenectDeviceFlags::CAMERA, FreenectDeviceFlags::MOTOR]);

    let n = context.num_devices ().unwrap ();

    println! ("Number of devices: {}", n);

    let mut dev = Device::open_device (&context, 0).unwrap ();

    let mut buffer = [0; 640*480*3];

    freenect_set_video_callback! (dev, fn callback (array : RGBArray, timestamp: u32) {
        pass_to_paint_thread (array);
    });

    dev.set_tilt_degs (0 as f64);
    dev.set_led (FreenectLedOptions::GREEN);
    dev.set_video_mode (FreenectFrameMode::find_video_mode (FreenectResolution::MEDIUM, FreenectVideoFormat::RGB));
    dev.set_video_buffer (&mut buffer);

    dev.start_video ();

    while context.process_events () == StatusCode::Success {
    }
}

fn pass_to_paint_thread (array : RGBArray) {

}
