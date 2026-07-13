# Thread Pool

A small Rust thread pool built as a concurrency practice project.

The pool creates a fixed number of worker threads. Jobs are stored in a shared
`VecDeque`, protected by a `Mutex` and shared between workers with `Arc`.

Workers wait on a `Condvar` when the queue is empty. When `execute` pushes a new
job into the queue, it notifies one worker so the job can be popped and run.

When the `ThreadPool` is dropped, it sets a shutdown flag, wakes all workers,
and joins their `JoinHandle`s so the program can exit cleanly.

Run it with:

```bash
cargo run
```

This is a learning project, not a production-ready thread pool.
