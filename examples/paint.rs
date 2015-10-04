use image::ImageBuffer;

let window : GlutinWindow = WindowSettings::new ("Test Freenect-rs", [640, 480])
                                .opengl (opengl)
                                .exit_on_esc (true)
                                .build ()
                                .unwrap ();

const opengl : OpenGL = OpenGL::V3_2;


fn paint (array : RGBArray, window : GlutinWindow) {
    let mut gl = GlGraphics::new (opengl);

    for e in window.events () {
        match e {
            Event::Render(args) => { render (args, &mut gl, &array); },
            _ => {},
        }
    }
}

fn render (args : RenderArgs, gl : &mut GlGraphics, array : &RGBArray) {
    use graphics::*;

    let viewport = args.viewport ();
    let img = ImageBuffer::new (640, 480);

    gl.draw (viewport, |c, gl| {
        for (pixel, dot) in array.iter ().zip(img.pixels ()) {
            let r : f64 = (.r as f64)/255.0;
            let g : f64 = (pixel.g as f64)/255.0;
            let b : f64 = (pixel.b as f64)/255.0;

            let color : [f64; 4] = [r, g, b, 1.0];

            dot = color;
        }

        draw (img);
    });
}
