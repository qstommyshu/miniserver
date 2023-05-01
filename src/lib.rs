use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
};

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}

// struct Job;
type Job = Box<dyn FnOnce() + Send + 'static>; //Type alias

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            // create some threads and store them in vector
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }
        ThreadPool {
            workers,
            sender: Some(sender),
        }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        // self.sender.send(job).unwrap();
        self.sender.as_ref().unwrap().send(job).unwrap();
    }

}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take());

        for worker in &mut self.workers {
            //for iterator get references, needs to chagne to mutable references
            println!("Shutting down worker {}", worker.id);

            // worker.thread.join().unwrap();
            //Take the ownership of worker that still have threads
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}
struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>, //what is ()? a joinHandle that doesn't return anything?
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            // receiver; //TODO: understand this??
            // receiver.get a lock. receive. unwrap info
            let message = receiver.lock().unwrap().recv();

            match message {
                Ok(job) => {
                    println!("Worker {id} got a job; executing.");

                    job();
                }
                Err(_) => {
                    println!("Worker {id} got disconnected; shutting down.");
                    break;
                }
            }
            // println!("Worker {id} got a job; executing.");

            // job();
        });
        Worker {
            id,
            thread: Some(thread),
        }
    }
}

