# rpc-agent

> A modular Rust library for building RPC-based agents with pluggable provider integrations.

---

## Overview

**rpc-agent** is a Rust crate that lets you build resilient, extensible agents which talk to LLM services over RPC. Leveraging the Rig framework, it supports multiple provider backends, currently OpenAI and Ollama, and is designed for straightforward extension and robust error handling.

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
rpc-agent = { path = "../rpc-agent" } # or use the published version if available
tokio = { version = "1.48.0", features = ["macros", "rt-multi-thread"] }
```

---

## Example Usage

Here is a minimal example of how to set up and run an Ollama RPC agent:

```rust
use rpc_agent::Providers;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let builder = rpc_agent::AgentServerBuilder::new(
        5500,
        Providers::Ollama,
        "You're a friendly assistant",
        "gpt-oss:20b",
    );

    let server = builder?.build()?;

    server.run().await?;

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
