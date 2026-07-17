use std::{
    sync::{Arc, Condvar, Mutex},
    thread::{self, JoinHandle, sleep},
    time::Duration,
};

use crate::inputs::{english_translation, random_quote};
mod inputs;
pub struct BoundedQueue {
    shared: Arc<Shared>,
    producer_threads: ProducerThreads,
    consumer_threads: ConsumerThreads,
}

impl BoundedQueue {
    pub fn new(producer_size: usize, consumer_size: usize) -> Self {
        assert!(
            producer_size > 0,
            "producer count must be greater than zero"
        );

        assert!(
            consumer_size > 0,
            "consumer count must be greater than zero"
        );
        let mut producer_threads = ProducerThreads::new(producer_size);
        let mut consumer_threads = ConsumerThreads::new(consumer_size);

        let shared = Arc::new(Shared::new());

        for producer in 0..producer_size {
            let shared_clone = Arc::clone(&shared);

            let handle = thread::spawn(move || {
                println!("spawned producer: {producer}");
                let shared_state = &*shared_clone;

                let Shared {
                    shared_state,
                    not_full,
                    not_empty,
                } = shared_state;

                loop {
                    let mut lock = shared_state.lock().unwrap();
                    while !lock.shutdown && !lock.queue.iter().any(|x| x.is_none()) {
                        lock = not_full.wait(lock).unwrap();
                    }

                    if lock.shutdown {
                        drop(lock);
                        println!("producer: {producer} is leaving!");
                        break;
                    } else if lock.queue.iter().any(|x| x.is_none()) {
                        let quote = random_quote();
                        if let Some(x) = lock.queue.iter().position(|x| x.is_none()) {
                            lock.queue[x] = Some(quote);
                            drop(lock);
                            not_empty.notify_one();
                        }
                    }
                }
            });

            producer_threads.0.push(Some(handle));
        }

        for consumer in 0..consumer_size {
            let shared_clone = Arc::clone(&shared);

            let handle = thread::spawn(move || {
                println!("spawned consumer: {consumer}");
                let shared_state = &*shared_clone;

                let Shared {
                    shared_state,
                    not_full,
                    not_empty,
                } = shared_state;

                loop {
                    let mut lock = shared_state.lock().unwrap();
                    while !lock.shutdown && !lock.queue.iter().any(|x| x.is_some()) {
                        lock = not_empty.wait(lock).unwrap();
                    }

                    if let Some(x) = lock.queue.iter().position(|x| x.is_some()) {
                        let y = lock.queue[x].unwrap();

                        lock.queue[x] = None;

                        lock.queue.rotate_left(1);

                        drop(lock);
                        not_full.notify_one();
                        let translation = english_translation(y);
                        println!(
                            "Consumer: {} stumbled upon ancient wisdom: \n {}",
                            consumer, y
                        );
                        println!("Which translates to: \n {translation}");
                        sleep(Duration::from_secs(1));
                    } else if lock.shutdown {
                        println!("Consumer: {} is leaving", consumer);
                        break;
                    }
                }
            });

            consumer_threads.0.push(Some(handle));
        }

        Self {
            shared,
            producer_threads,
            consumer_threads,
        }
    }

    pub fn shutdown(mut self) {
        let mut lock = self.shared.shared_state.lock().unwrap();
        lock.shutdown = true;

        drop(lock);
        self.shared.not_full.notify_all();
        self.shared.not_empty.notify_all();

        for i in self.producer_threads.0.drain(..) {
            i.unwrap().join().unwrap();
        }

        for i in self.consumer_threads.0.drain(..) {
            i.unwrap().join().unwrap();
        }
    }
}
struct ProducerThreads(HandlesVec);

impl ProducerThreads {
    fn new(num: usize) -> Self {
        Self(Vec::with_capacity(num))
    }
}

struct ConsumerThreads(HandlesVec);

impl ConsumerThreads {
    fn new(num: usize) -> Self {
        Self(Vec::with_capacity(num))
    }
}
struct Shared {
    shared_state: Mutex<SharedState>,
    not_full: Condvar,
    not_empty: Condvar,
}

impl Shared {
    fn new() -> Self {
        Self {
            shared_state: Mutex::new(SharedState::new()),
            not_full: Condvar::new(),
            not_empty: Condvar::new(),
        }
    }
}
struct SharedState {
    queue: [Option<&'static str>; 1],

    shutdown: bool,
}
impl SharedState {
    fn new() -> Self {
        Self {
            queue: [const { None }; 1],

            shutdown: false,
        }
    }
}
type HandlesVec = Vec<Option<JoinHandle<()>>>;
