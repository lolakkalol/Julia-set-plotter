use sdl2::{
    pixels::Color, 
    rect::Point, 
    keyboard::Keycode,
    event::Event
};

use complex_values::Complex;
use julia::JuliaSet;

// Used to avoid bad roundings in floating point arithmetic
fn round(x: f64) -> i32{
    let y = x - (x as i32) as f64;

    if y >= 0.5 {
        return (x + 0.5) as i32;
    }

    x as i32
}

pub fn main() {
    let (window_width, window_height):(u32, u32) = (1000, 1000);
    
    let sdl_context = sdl2::init().unwrap();
    let video_system = sdl_context.video().unwrap();
    let window = video_system
    .window("This is a window", window_width, window_height)
    .build()
    .unwrap();

    let mut canvas = window.into_canvas().software().build().unwrap();

    let mut event_pump = sdl_context.event_pump().unwrap();

    let min = Complex(-1.0, -1.0);
    let max = Complex(1.0, 1.0);
    let constant = Complex(-1.0, 0.1);
    let resolution = (max.0-min.0)/window_width as f64;

    let mut julia = JuliaSet::new(min, max,  constant, resolution);

    let mut slide: f64 = 0.0;

    'running: loop {
        
        // Checks for events
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit{..} |
                Event::KeyDown {keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }

        // Draws the background black
        canvas.set_draw_color(Color{r: 0, g: 0, b: 0, a:255});
        canvas.clear();

        julia.set_constant(Complex(slide, 0.4));
        let set = julia.calculate().take().unwrap();

        slide += 0.001;

        for (x, y, a) in set {
            canvas.set_draw_color(Color{r: ((255/100))*a as u8, g: ((255/100))*a as u8, b: ((255/100))*a as u8, a:255});
            canvas.draw_point(Point::new(
            round((x-min.0)*(window_width as f64)/(max.0-min.0)), 
            round((y-min.1)*(window_height as f64)/(max.1-min.1)))
            ).unwrap();
        }
        canvas.present();

    }
}