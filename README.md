# Rust RabbitMQ Worker Template

A **production-ready RabbitMQ worker template written in Rust**, built on **Tokio** and **Lapin**, supporting both **single-message** and **batch-based** consumer patterns.


## Features

* Async Rust using Tokio (multi-threaded runtime)
* RabbitMQ consumer using Lapin
* Two consumer modes:
  * **Single-message processing**
  * **Batch processing with QoS (prefetch)**
* Explicit ACK / NACK semantics
* Backpressure-safe via RabbitMQ QoS
* Header access and decoding
* Configurable via environment variables
* Optional Docker and docker-compose support
* Clean, minimal structure



## Repository Structure

```
.
├── src-single/        # Single-message consumer
│   ├── main.rs
│   └── rabbitmq.rs
│
├── src-batch/         # Batch consumer
│   ├── main.rs
│   └── rabbitmq.rs
│
├── Cargo.toml
├── Dockerfile
├── docker-compose.yaml
├── README.md
└── LICENSE
```

### Which one should I use?

* Use **`src-single`** if:
  * Each message is processed independently
  * Processing is fast and simple

* Use **`src-batch`** if:
  * You want to process messages in groups
  * You are writing to DBs, external APIs, or doing vectorized work
  * You need batch-level ACK semantics

* Rename the chosen directory to `src/` to use it as the main source. And delete the other one if you want.



## Configuration (Environment Variables)

All configuration is done via environment variables.

### Logging

```bash
RUST_LOG=rust_rmq_worker=TRACE
```



### RabbitMQ Connection

```bash
RABBITMQ_HOST=amqp.nekonik.com
RABBITMQ_PORT=5672
RABBITMQ_USERNAME=nekonik
RABBITMQ_PASSWORD=NekoNik
RABBITMQ_VHOST=nekonik_vhost
```



### Consumer Settings

```bash
RABBITMQ_CONSUMER_TAG=rust_consumer
RABBITMQ_QUEUE_NAME=nekonik_queue
RABBITMQ_QUEUE_DURABLE=true
```



### Batch Processing (Only if you pick `src-batch`)

```bash
RABBITMQ_PREFETCH_COUNT=10
RABBITMQ_PREFETCH_WINDOW_ms=200
```

* `RABBITMQ_PREFETCH_COUNT`
  * RabbitMQ QoS prefetch
  * Maximum number of unacked messages

* `RABBITMQ_PREFETCH_WINDOW_ms`
  * Batch collection window
  * Controls how long the worker waits to fill a batch (in milliseconds)



## Running Locally (No Docker)

After picking either `src-single` or `src-batch` as your source directory, and setting up environment variables, you can run the worker using simple Cargo commands.

```bash
cargo run
```



## Docker Compose (Expected Environment Section)

```yaml
environment:
  # Logging
  RUST_LOG: rust_rmq_worker=TRACE

  # RabbitMQ
  RABBITMQ_HOST: amqp.nekonik.com
  RABBITMQ_PORT: 5672
  RABBITMQ_USERNAME: nekonik
  RABBITMQ_PASSWORD: NekoNik
  RABBITMQ_VHOST: nekonik_vhost

  # Consumer
  RABBITMQ_CONSUMER_TAG: rust_consumer
  RABBITMQ_QUEUE_NAME: nekonik_queue
  RABBITMQ_QUEUE_DURABLE: true

  # Batch only
  RABBITMQ_PREFETCH_COUNT: 10
  RABBITMQ_PREFETCH_WINDOW_ms: 200
```


## Message Acknowledgement Semantics

### Single consumer

* Each message is ACKed after successful processing

### Batch consumer

* Messages are:
  1. Collected up to `PREFETCH_COUNT`
  2. Processed as a batch
  3. ACKed **after batch success**

* If batch processing fails:
  * Messages should be NACKed (optionally requeued)


## Contributing

Contributions are welcome! If you'd like to contribute to Rust-RabbitMQ-Worker Template, please follow these steps:

1. Fork the repository
2. Create a new branch for your feature or bug fix
3. Make your changes and commit them
4. Push your changes to your fork
5. Submit a pull request to the `main` branch of the original repository

Please make sure to follow the existing code style and add tests for any new features or bug fixes.

## License

Rust-RabbitMQ-Worker Template is released under the [MIT License](https://github.com/Neko-Nik-Org/Rust-RabbitMQ-Worker-Template/blob/main/LICENSE). You are free to use, modify, and distribute this template for any purpose.
