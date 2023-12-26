/*

//#[warn(unused_imports)]
use std::thread;
use std::net::TcpListener;
use std::io::prelude::*;
//extern crate threadpool;
//use threadpool::ThreadPool;
//use thread;
//use std::os::windows::thread;

//use std::thread;
//use std::ThreadPool;
/*
fn main() {
    let listener = TcpListener::bind("0.0.0.0:8080").unwrap();

    for stream in listener.incoming() {
        let mut stream = stream.unwrap();

        let mut buffer = [0; 1024];
        stream.read(&mut buffer).unwrap();

        let message = String::from_utf8_lossy(&buffer).to_string();
        println!("Received message: {}", message);

        stream.write(message.as_bytes()).unwrap();
    }
}

 */

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    let mut pool = std::thread::ThreadPool::new(10);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.spawn(move || {
            let mut buffer = [0; 1024];
            stream.read(&mut buffer).unwrap();

            let message = String::from_utf8_lossy(&buffer).to_string();
            println!("Received message: {}", message);

            stream.write(message.as_bytes()).unwrap();
        });
    }

    // 按下 `Ctrl+C` 时，线程池将被关闭
    pool.shutdown();
}

*/

use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::thread;

const NUM_THREADS: usize = 4;

struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool { workers, sender }
    }

    fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.send(job).unwrap();
    }
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let job = receiver.lock().unwrap().recv().unwrap();
            job();
        });

        Worker {
            id,
            thread: Some(thread),
        }
    }
}

fn main() {
    let pool = ThreadPool::new(NUM_THREADS);

    for i in 0..10 {
        pool.execute(move || {
            println!("Task {} executed by thread {:?}", i, thread::current().id());
        });
    }

    // Ensure all threads finish their tasks
    thread::sleep(std::time::Duration::from_secs(2));
}
