use std::{thread::sleep, time::Duration};

mod structs;
use structs::ThreadPool;

fn main() {
    let pool = ThreadPool::new(5);
    let secret = Box::new(move || println!("printing the secret num..."));

    pool.execute(secret);

    sleep(Duration::from_secs(5));

    let num = Box::new(move || println!("....is 25"));

    pool.execute(num);

    // pool.shutdown();
}
