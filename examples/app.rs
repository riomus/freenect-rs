use freenect::buffer::RGBBufferVideoMedium;
use im::{ImageBuffer, Rgba};
use opengl_graphics::{GlGraphics, Texture, OpenGL};
use piston_window::PistonWindow;
use piston::window::WindowSettings;
use piston::event_loop::{Events, EventLoop};
use piston::input::RenderEvent;
use graphics::{clear, Image, default_draw_state};
use std::sync::{Arc, Mutex};

const OPENGL : OpenGL = OpenGL::V3_2;
const WIDTH : u32 = 640;
const HEIGHT : u32 = 480;

pub struct App {
    pub window  : PistonWindow,
    pub texture : Texture,
}

impl App {
    pub fn new () -> App {
        let window =
            WindowSettings::new("Freenect-rs example", (WIDTH, HEIGHT))
            .exit_on_esc(true)
            .opengl(OPENGL)
            .build()
            .unwrap();
        let canvas = ImageBuffer::new(WIDTH, HEIGHT);
        let texture = Texture::from_image (&canvas);
        App {
            window : window,
            texture : texture,
        }
    }

    pub fn init (&mut self, canvas : Arc<Mutex<ImageBuffer<Rgba<u8> ,Vec<u8>>>>) {
        let mut gl = GlGraphics::new (OPENGL);
        let img = Image::new ();
        let mut event_loop = self.window.clone().window.events ();
        for e in event_loop {
            if let Some (r) = e.render_args () {
                let canvas = canvas.lock ().unwrap ();
                self.texture.update (&canvas);
                gl.draw (r.viewport (), |c, gl| {
                    clear ([1.0; 4], gl);
                    img.draw (&self.texture, default_draw_state (), c.transform, gl);
                });
            }
        }
    }
}
