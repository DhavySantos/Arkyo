# 🚀 Rust HTTP Server Library

A lightweight and fast HTTP server library written in Rust, designed for routing, middleware handling, and request/response processing. Perfect for building your own web server with customizable features.

## ✨ Features

- **🌐 Routing**: Supports GET, POST, PATCH, DELETE methods with dynamic routing.
- **🧩 Middleware**: Add middleware functions to process requests before they reach routes.
- **📜 Request Parsing**: Parse HTTP requests (headers, body, URL parameters).
- **💬 Response Handling**: Send HTTP responses with status codes, headers, and body.
- **🛠️ Dynamic Location Matching**: Route matching with dynamic parameters in the URL (e.g., `/user/:id`).

---

## 📚 Quick Start

### 💻 Install

To add this library to your project, add it to your `Cargo.toml`:

```toml
[dependencies]
arkyo = "0.0.7"
```

## Example Usage
```rust
use your_http_server_lib::Server;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut server = Server::new();

    server.get("/hello", |req, res| {
        res.body("Hello, World! 👋");
        res.send();
    })?;

    server.post("/submit", |req, res| {
        res.body("Form submitted! ✔️");
        res.send();
    })?;

    server.add_middleware("/hello", |req, res| {
        println!("Request to /hello received. 🚪");
    })?;

    server.listen("127.0.0.1:8080");
    Ok(())
}
```

## 🛣️ Roadmap
- [x] 🧩 WebSocket support.
- [ ] 🔒 HTTPS support.
- [ ] 📈 Persistent connection handling.
- [ ] 📝 Improved error handling & logging.
- [ ] 💡 Request/Response body parsing (JSON, FormData, etc.).
- [ ] 🌐 HTTP/3 support.

