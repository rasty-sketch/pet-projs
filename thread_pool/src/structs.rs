use std::{
    collections::VecDeque,
    ops::RangeFull,
    sync::{Arc, Condvar, Mutex},
    thread::{self, JoinHandle},
};

pub struct ThreadPool {
    threads: Vec<Option<JoinHandle<()>>>,
    shared: Arc<Shared>,
}

struct Shared {
    shared_state: Mutex<SharedState>,
    cvar: Condvar,
}

impl Shared {
    fn new() -> Self {
        Self {
            shared_state: SharedState::mutex_state(),
            cvar: Condvar::new(),
        }
    }
}

struct SharedState {
    jobs: VecDeque<Job>,
    shutdown: bool,
}

impl SharedState {
    fn new() -> Self {
        let jobs = VecDeque::<Job>::new();
        let shutdown = false;
        Self { jobs, shutdown }
    }

    fn mutex_state() -> Mutex<Self> {
        Mutex::new(SharedState::new())
    }
}

impl ThreadPool {
    pub fn new(size: usize) -> Self {
        assert!(size > 0, "Thread pool size must be greater than zero");

        let mut threads = Vec::with_capacity(size);
        let shared = Arc::new(Shared::new());

        for i in 0..size {
            let shared_clone = Arc::clone(&shared);
            let handle = thread::spawn(move || {
                println!("spawned thread: {i}");
                let shared_state = &*shared_clone;
                let Shared {
                    shared_state: mutex,
                    cvar: condvar,
                } = shared_state;
                loop {
                    let mut lock = mutex.lock().unwrap();

                    while lock.shutdown == false && lock.jobs.len() == 0 {
                        println!("there are no jobs!");
                        lock = condvar.wait(lock).unwrap();
                    }

                    if let Some(job) = lock.jobs.pop_front() {
                        println!("got some job..... -thread: {i}");
                        drop(lock);
                        job();
                    } else if lock.shutdown {
                        drop(lock);
                        println!("all good things must come to an end! \n -thread: {}", i);
                        break;
                    }
                }
            });

            threads.push(handle);
        }

        let option_threads = threads.into_iter().map(|x| Some(x)).collect();

        Self {
            threads: option_threads,
            shared,
        }
    }

    pub fn execute(&self, job: Job) {
        let mut lock = self.shared.shared_state.lock().unwrap();
        lock.jobs.push_back(job);
        drop(lock);
        self.shared.cvar.notify_one();
    }

    pub fn _shutdown(mut self) {
        let mut lock = self.shared.shared_state.lock().unwrap();
        lock.shutdown = true;

        drop(lock);
        self.shared.cvar.notify_all();

        for i in self.threads.drain(RangeFull) {
            i.unwrap().join().unwrap();
        }
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        let mut lock = self.shared.shared_state.lock().unwrap();

        lock.shutdown = true;

        drop(lock);

        self.shared.cvar.notify_all();

        for i in &mut self.threads {
            if let Some(handle) = i.take() {
                handle.join().unwrap();
            }
        }
    }
}

pub type Job = Box<dyn FnOnce() + Send + 'static>;
