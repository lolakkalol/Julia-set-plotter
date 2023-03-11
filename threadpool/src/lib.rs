use std::sync::{Arc, Mutex};
use std::sync::mpsc::{self, Sender, Receiver};
use std::thread;

type Job = Box<dyn FnOnce() + Send + 'static>;

// Will hold thread and an ID
struct Worker {
    id    : usize,
    thread: Option<thread::JoinHandle<()>>
}

impl Worker {
    fn new(id: usize, receiver: Arc< Mutex< Receiver<Job> > >) -> Worker {

        let thread = thread::spawn( move || loop {
            let message = receiver.lock().unwrap().recv();
            
            match message {
                Ok(job) => {
                    job();
                }
                Err(_) => {
                    break;
                }
            }
        });

        Worker{
            id,
            thread: Some(thread)
        }
    }
}

pub struct ThreadPool {
    workers : Vec<Worker>,
    // Is an option so we can move sender out of the struct and other
    // functions like drop can take ownership of sender.
    sender  : Option<Sender<Job>> 
}

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {

        // Initilize vector of specific size: usize
        let mut workers = Vec::with_capacity(size);

        // Create a communication channel between the main thread and slave threads
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new( Mutex::new( receiver ) );

        // Create and store workers
        for i in 0..size {
            workers.push( Worker::new(i, Arc::clone(&receiver)) );
        }

        // Returns the threadpool struct to be used
        ThreadPool{
            workers,
            sender: Some(sender)
        }
    }

    pub fn execute<T: FnOnce() + Send + 'static>(&self, function: T) {
        let function = Box::new(function);

        // Send a closure to an avaliable thread
        self.sender.as_ref().unwrap().send( function ).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take());

        // Wait for threads to finish
        for worker in &mut self.workers {
            worker.thread.take().unwrap().join().unwrap();
        }
    }
    
}