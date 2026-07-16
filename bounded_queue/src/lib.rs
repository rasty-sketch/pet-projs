/// producer Threads -> Queue fixed size,cant grow, i could do array or vec and check len(), -> consumer threads;
/// ill use &str in v1,then make item generic, itd be like writing to memory and consuming on the other end,
/// ill use option(&str) to replace the item with None when consumed, so compiler wont complain,
/// desired output states:
/// 1-{[None,None,None]}, consumer_threads and producers start running,
/// 2-consumer cant require lock because is_empty, then a function runs
/// thatll will take &str as an argument, and the shared state, and acquires the lock
/// Queue {
///     Shared{
///         shared_vars: Arc(of queue,shutdown,is_empty,is_full);
///         condvar: to keep checking the shared state
///     },
///     consumer Threads,
///     producer Threads
/// }
///
///
/// new design idea is that itd produce items randomly lets say ill make an array of 10 strings and
/// everytime theres room itll choose between one of the 10 quotes, to put it into the queue array,
/// so it has to be static global maybe
/// and then itd be consumed by the consumer threads
/// maybe having a match would be make for prettier output, consumers can be translators,
/// ill get 10 latin quotos and there translation to english,
/// thats to come later now ill design the loops each of producer and consumer would be running on
use std::{
    sync::{Arc, Condvar, Mutex},
    thread::{self, JoinHandle},
};

use crate::inputs::{english_translation, random_quote};
mod inputs;
pub struct BoundedQueue<'a> {
    shared: Arc<Shared<'a>>,
    producer_threads: ProducerThreads,
    consumer_threads: ConsumerThreads,
}

impl BoundedQueue<'_> {
    pub fn new(producer_size: usize, consumer_size: usize) -> Self {
        assert!(
            producer_size > 0,
            "thread pool soze must be greater than zero"
        );

        assert!(
            consumer_size > 0,
            "thread pool soze must be greater than zero"
        );
        let mut producer_threads = ProducerThreads::new(producer_size);
        let mut consumer_threads = ConsumerThreads::new(consumer_size);

        let shared = Arc::new(Shared::new());
        // let shared_cl = Arc::clone(&shared);

        for producer in 0..producer_size {
            let shared_clone = Arc::clone(&shared);

            let handle = thread::spawn(move || {
                println!("spawned producer: {producer}");
                let shared_state = &*shared_clone;

                let Shared { shared_state, cvar } = shared_state;

                loop {
                    let mut lock = shared_state.lock().unwrap();
                    while !lock.shutdown && lock.is_full {
                        lock = cvar.wait(lock).unwrap();
                    }

                    if lock.shutdown {
                        drop(lock);
                        println!("producer: {producer} is leaving!")
                    } else if !lock.is_full {
                        let quote = random_quote();
                        if let Some(x) = lock.queue.iter().position(|x| matches!(x, None)) {
                            lock.queue[x] = Some(quote);
                            if !lock.queue.contains(&None) {
                                lock.is_full = true;
                            } else {
                                lock.is_full = false;
                            }
                            drop(lock);
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

                let Shared { shared_state, cvar } = shared_state;

                loop {
                    let mut lock = shared_state.lock().unwrap();
                    while !lock.shutdown && lock.is_emtpy {
                        lock = cvar.wait(lock).unwrap();
                    }

                    if !lock.is_emtpy {
                        if let Some(x) = lock.queue.iter().position(|x| x.is_some()) {
                            let y = lock.queue[x];

                            lock.queue[x] = None;

                            lock.queue.rotate_left(1);

                            if lock.queue.iter().any(|x| x.is_some()) {
                                lock.is_emtpy = false;
                            } else {
                                lock.is_emtpy = true
                            }
                            drop(lock);
                            let translation = english_translation(y.unwrap());
                            println!(
                                "Consumer: {} stumbled upon ancient wisdom: \n {}",
                                consumer,
                                y.unwrap()
                            );
                            println!("Which translates to: \n {translation}");
                        } else if lock.shutdown {
                            println!("Consumer: {} is leaving", consumer);
                        }
                    }
                }
            });

            consumer_threads.0.push(Some(handle));
        }

        // Self {
        //     shared: Arc::new(Shared::new()),
        //     producer_threads: ProducerThreads::new(5),
        //     consumer_threads: ConsumerThreads::new(5),
        // }
        Self {
            shared,
            producer_threads,
            consumer_threads,
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
struct Shared<'a> {
    shared_state: Mutex<SharedState<'a>>,
    cvar: Condvar,
}

impl Shared<'_> {
    fn new() -> Self {
        Self {
            shared_state: Mutex::new(SharedState::new()),
            cvar: Condvar::new(),
        }
    }
}
struct SharedState<'a> {
    queue: [Option<&'a str>; 3],
    is_emtpy: bool,
    is_full: bool,
    shutdown: bool,
}
impl SharedState<'_> {
    fn new() -> Self {
        Self {
            queue: [const { None }; 3],
            is_emtpy: true,
            is_full: false,
            shutdown: false,
        }
    }
}
type Job = Box<dyn FnOnce() + Send + 'static>;
type HandlesVec = Vec<Option<JoinHandle<()>>>;
