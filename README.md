# grpc-agent

> A modular Rust library for building gRPC-based agents with pluggable provider integrations.

---

## Overview

**grpc-agent** is a Rust library crate for building robust, extensible agents that communicate over gRPC. It supports multiple provider backends (like OpenAI and Ollama) and is structured for easy extension and error handling.

---

## Features

- Modular agent architecture for easy extension
- Pluggable provider integrations (Ollama, OpenAI, etc.)
- Centralized error handling
- Async support (Tokio-based)
- Written in idiomatic Rust

---

## Getting Started

### Prerequisites

- Rust (latest stable recommended)
- [Cargo](https://doc.rust-lang.org/cargo/)
- [Tokio](https://crates.io/crates/tokio)

Add this crate to your `Cargo.toml`:

```toml
[dependencies]
grpc-agent = { path = "../grpc-agent" } # or use the published version if available
tokio = { version = "1.48.0", features = ["macros", "rt-multi-thread"] }
```

---

## Example Usage

Here is a minimal example of how to set up and run an Ollama gRPC agent:

```rust
use grpc_agent::Providers;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let agent = grpc_agent::Agent::new(
        5500, // Port
        Providers::Ollama,
        "You're a friendly assistant", // System prompt
        "gpt-oss:20b", // Model name
        None, // API key (if needed)
        None, // Temperature of range 0.0 - 1.0
        None, // Max Tokens
    )?;

    agent.run().await?;
    Ok(())
}
```

> **Note:** Adjust the parameters as needed for your provider and use case. See the source files for more details and available options.

---

## Contributing

Pull requests and issues are welcome! Please open an issue to discuss your ideas or report bugs.

---

## License

This project is licensed under the MIT License.
