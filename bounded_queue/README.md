# Bounded Producer-Consumer Queue

A small Rust project built to learn shared-memory concurrency and thread synchronization.

## What It Does

- Starts multiple producer and consumer threads.
- Shares one fixed-capacity FIFO buffer between them.
- Blocks producers while the buffer is full.
- Blocks consumers while the buffer is empty.
- Uses separate condition variables for available space and available items.
- Rejects new items during shutdown.
- Drains queued items before consumers exit.
- Joins all worker threads cleanly.

The demo uses Latin quotes as produced items and prints their English translations when consumed.

## Running The Demo

```bash
cargo run
```

The binary starts the queue with multiple producers and consumers, lets them run briefly, then shuts the queue down cleanly.

## Project Layout

- `src/lib.rs` contains the bounded queue, shared state, condition variables, worker thread loops, and shutdown logic.
- `src/inputs.rs` contains the Latin quote inputs and English translations used by the demo.
- `src/main.rs` starts the demo queue and triggers shutdown.
