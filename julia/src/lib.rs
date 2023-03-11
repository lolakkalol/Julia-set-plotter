use complex_values::Complex;
use threadpool::ThreadPool;
use std::sync::mpsc::{Sender, Receiver};
use std::sync::{mpsc, Mutex, Arc};

pub struct JuliaSet {
    resolution: f64,
    min: Complex,
    max: Complex,
    constant: Complex,

    // The thread pool is saved to prevent it being droped prematurely
    pool: ThreadPool,
    calc_sender: Option<Sender<(Complex, Complex, Complex)>>,
    receiver:    Option<Receiver<Vec<(f64, f64, u32)>>>,
}

impl JuliaSet {
    pub fn new(min: Complex, max: Complex, constant: Complex, resolution: f64) -> JuliaSet {
        let resolution = resolution;

        let pool = ThreadPool::new(12);

        // Communication channel to delegate work to calculation threads
        let (calc_sender, calc_receiver) = mpsc::channel();
        let calc_receiver = Arc::new(Mutex::new(calc_receiver));

        // Communication cannel to receive finished calculations
        let (sender, receiver) = mpsc::channel();
        let sender = Arc::new( Mutex::new ( sender ) );

        // Setting up calculation threads
        for _ in 0..12 {
            let calc_receiver = Arc::clone(&calc_receiver);
            let sender = Arc::clone(&sender);
    
            pool.execute(move || loop {
                let message = calc_receiver.lock().unwrap().recv();
        
                match message {
                    Ok((min, max, constant)) => {
                        let set = calculate_julia_set(resolution, min, max, constant);
                        sender.lock().unwrap().send(set).unwrap();
                    },
                    // Break when sender is dropped or other error occured
                    Err(_) => {
                        break;
                    }
                }
            });
        }

        let receiver = Some(receiver);
        let calc_sender = Some(calc_sender);

        JuliaSet { resolution, min, max, constant, pool, calc_sender, receiver }
    }

    pub fn calculate(&self) -> Option<Vec<(f64, f64, u32)>> {
        let steps = f64::ceil((self.max.0 - self.min.0)/(12.0*self.resolution));

        for i in 0..12 {
            let min = Complex(self.min.0 + i as f64 * self.resolution * steps, -1.0);
            let mut max_bound = self.min.0 + (i as f64 + 1.0) * self.resolution * steps;
            if max_bound > self.max.0 {
                max_bound = self.max.0;
            }
            let max = Complex(max_bound, 1.0);

            self.calc_sender.as_ref().unwrap().send((min, max, self.constant)).unwrap();
        }
        let mut points = Vec::with_capacity(0);
        
        
        for _ in 0..12 {
            let mut message = self.receiver.as_ref().unwrap().recv().unwrap();
            points.append( &mut message );
        }
        
        if points.len() <= 0 {
            return None;
        }

        Some(points)
    }
        
    pub fn set_max(&mut self, max: Complex) {
        self.max = max;
    }

    pub fn set_min(&mut self, min: Complex) {
        self.min = min;
    }

    pub fn set_resolution(&mut self, resolution: f64) {
        self.resolution = resolution;
    }

    pub fn set_constant(&mut self, constant: Complex) {
        self.constant = constant;
    }
}

impl Drop for JuliaSet {
    fn drop(&mut self) {
        drop(self.receiver.take());
        drop(self.calc_sender.take());
    }
}
fn next_z(current: Complex, constant: Complex) -> Complex {
    current*current + constant
}

fn is_in_julia_set(z: Complex, constant: Complex) -> Option<u32> {
    let mut z = z;

    for iter in 0..100 {
        if z.abs() > 2.0 {
            return Some(iter);
        }

        z = next_z(z, constant);

    }

    None
}

fn calculate_julia_set(resolution: f64, min: Complex, max: Complex, constant: Complex) -> Vec<(f64, f64, u32)> {
    let steps_x = round((max.0-min.0)/resolution);
    let steps_y = round((max.1-min.1)/resolution);

    let mut set = Vec::<(f64, f64, u32)>::new();

    for x in 0..steps_x {
        for y in 0..steps_y {

            // Convert the step into floats to use for calculations
            let x = x as f64 * resolution + min.0;
            let y = y as f64 * resolution + min.1;

            let is_in_set = is_in_julia_set(
                    Complex(x, y), 
                    constant
            );

            match is_in_set {
                Some(iter) => set.push((x, y, iter)),
                _ => {}
            }
        }
    }

    set
}

fn round(x: f64) -> i32{
    let y = x - (x as i32) as f64;

    if y >= 0.5 {
        return (x + 0.5) as i32;
    }

    x as i32
}