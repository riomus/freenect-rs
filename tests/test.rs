#[macro_use]
extern crate freenect;
extern crate libc;
extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

use freenect::ffi::*;
use freenect::context::Context;
use freenect::device::{Device, RGBArray};
use opengl_graphics::{GlGraphics, OpenGL};
use piston::window::WindowSettings;
use glutin_window::GlutinWindow;
use piston::event_loop::*;
use piston::input::*;

fn main () {
    let mut context = Context::init (None).unwrap ();

    context.set_log_level (FreenectLogLevel::DEBUG);

    freenect_set_log_callback! (context, fn cb (log_level : FreenectLogLevel, m : &str) {
        println! ("{}", m);
    });

    let n = context.num_devices ().unwrap ();

    println! ("Number of devices: {}", n);

    let mut dev = Device::open_device (&context, 0).unwrap ();

    freenect_set_video_callback! (dev, fn callback (array : RGBArray, timestamp: u32) {
        for p in array.iter () {
            println! ("{:?}", p);
        }
    });

    let opengl = OpenGL::V3_2;

    let window : GlutinWindow = WindowSettings::new ("Test Freenect-rs", [640, 480])
                                    .opengl (opengl)
                                    .exit_on_esc (true)
                                    .build ()
                                    .unwrap ();

    let mut gl = GlGraphics::new (opengl);

    for e in window.events () {
        match e {
            Event::Render(args) => { render (args, &mut gl); },
            _ => {},
        }
    }
}

fn render (args : RenderArgs, gl : &mut GlGraphics) {
    use graphics::*;

    let viewport = args.viewport ();
    gl.draw (viewport, |c, gl| {
        const RED:   [f32; 4] = [1.0, 0.0, 0.0, 1.0];
        clear (RED, gl);
    });
}
