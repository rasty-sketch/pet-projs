use std::{thread, time::Duration};

use bounded_queue::BoundedQueue;

fn main() {
    let bounded_queue = BoundedQueue::new(5, 5);

    thread::sleep(Duration::from_secs(5));

    bounded_queue.shutdown();
}
