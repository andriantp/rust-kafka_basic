# 📘 rust-kafka_basic

> 🚀 Simple Kafka Producer & Consumer in Rust (pure-Rust, no librdkafka binding)

`rust-kafka_basic` is a minimal example project that implements a Kafka Producer and Consumer using Rust, relying on a pure-Rust Kafka client (i.e. no bindings to `librdkafka`).

This project is intended for:
- Exploring Kafka + Rust
- Proof-of-Concept (PoC) / Prototyping
- Learning the fundamentals of event-driven messaging / streaming systems

---

## 🔄 Operation Modes

This project supports two operation modes:

- **Producer** → publish messages to a Kafka topic  
- **Consumer** → consume messages from a Kafka topic  

All components can be run end-to-end using Docker Compose, and include :contentReference[oaicite:0]{index=0} for monitoring topics and messages.

---

## 🔐 Environment Variables (`.env`)

Create a `.env` file in the project root and set:

```bash
KAFKA_BROKER=localhost:9092
KAFKA_TOPIC=kafka-rust
KAFKA_GROUP_ID=group-1
```

---

## 🐳 Docker Compose

Features:
- Kafka in KRaft mode (i.e. without ZooKeeper)
- AKHQ UI for inspecting topics, message flow, and consumer groups

---

#### ▶️ Start Kafka & AKHQ
```bash
make up
```

---
#### 🌐 Access AKHQ
http://localhost:8080


---
## 📦 Build & Run
#### 🔧 Build
```bash
cargo build
```

#### 🚀 Run Producer
```bash
cargo run -- producer
```

#### 📥 Run Consumer
```bash
cargo run -- consumer
```

---
## 📁 Project Structure
```bash
rust-kafka_basic/
├── src/
│   ├── config.rs        # Kafka & environment configuration
│   ├── producer.rs      # Kafka producer logic
│   ├── consumer.rs      # Kafka consumer logic
│   └── main.rs          # Application entry point (CLI)
├── docker-compose.yml   # Kafka + AKHQ setup for development
├── .env                 # Environment configuration (ignored by git)
└── README.md
```

---
## ⚠️ Notes & Limitations

- The project uses a pure-Rust Kafka client
- Suitable for:
```bash
- Experimentation
- Learning Kafka internals with Rust
- Lightweight or non-critical systems
```
- Not intended for production-grade use. Key limitations:
```bash
- No support for transactions / exactly-once semantics
- No idempotent producer
- No advanced error handling, retries, or resilience mechanisms
```
If you need a production-grade Kafka client in Rust, it is recommended to use a more mature library such as rust-rdkafka (which uses librdkafka under the hood).


---
## 🔗 Referensi

Full article:
[Building a Simple Kafka Producer and Consumer in Rust](https://andriantriputra.medium.com/be-rust-kafka-building-a-simple-kafka-producer-and-consumer-in-rust-ed1e0b6051ba)

---

## Author

Andrian Tri Putra
- [Medium](https://andriantriputra.medium.com/)
GitHub
- [andriantp](https://github.com/andriantp)
- [AndrianTriPutra](https://github.com/AndrianTriPutra)

---

## License
Licensed under the Apache License 2.0