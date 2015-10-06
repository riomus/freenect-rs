use freenect::device::RGBArray;
use im::{ImageBuffer, Rgba};
use piston_window::*;
use std::thread;
use std::sync::{Arc, Mutex};
use std::sync::mpsc::Receiver;
use std::cell::RefCell;
use std::fmt::{Display, Formatter, Result};
use gfx_device_gl;

const OPENGL : OpenGL = OpenGL::V3_2;
const WIDTH : u32 = 640;
const HEIGHT : u32 = 480;

pub struct App {
    pub window  : PistonWindow,
    pub canvas  : Arc<Mutex<ImageBuffer<Rgba<u8> ,Vec<u8>>>>,
    pub texture : RefCell<Texture<gfx_device_gl::Resources>>,
}

impl App {
    pub fn new () -> App {
        let window : PistonWindow = WindowSettings::new("Freenect-rs example", (WIDTH, HEIGHT))
                                        .exit_on_esc(true)
                                        .opengl(OPENGL)
                                        .build()
                                        .unwrap();
        let canvas = ImageBuffer::new(WIDTH, HEIGHT);
        let texture = RefCell::new (Texture::from_image(
                        &mut *window.factory.borrow_mut(),
                        &canvas,
                        &TextureSettings::new()).unwrap ());

        App {
            window : window,
            canvas : Arc::new (Mutex::new (canvas)),
            texture : texture,
        }
    }

    pub fn init (&self, receiver : Receiver<&mut RGBArray>) {
        for e in self.window.clone() {
            e.draw_2d(|c, g| {
                let texture = self.texture.borrow ();
                clear([1.0; 4], g);
                image(&*texture, c.transform, g);
            });
            let image_ref = self.canvas.clone ();
            let mut image_buffer = &mut *image_ref.lock ().unwrap ();
            let screen = receiver.recv ().unwrap ();
            App::update_buffer (image_buffer, screen);
            self.texture.borrow_mut ().update (&mut *e.factory.borrow_mut(), image_buffer).unwrap();
        }
    }

    pub fn update_buffer (canvas : &mut ImageBuffer<Rgba<u8> ,Vec<u8>>, array : &mut RGBArray) {
        for (color, pixel) in array.iter ().zip (canvas.pixels_mut ()) {
            let color : [u8; 4] = [color.r, color.g, color.b, 255];
            *pixel = Rgba (color);
        }
    }
}
