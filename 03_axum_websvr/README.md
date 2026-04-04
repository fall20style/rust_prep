# Axum Static Web Server

A simple static file server built with [Axum](https://github.com/tokio-rs/axum) and [tower-http](https://github.com/tower-rs/tower-http). This project demonstrates how to serve static assets like HTML, CSS, and images.

## Features

- **Static File Serving**: Serves all files located in the `./static` directory.
- **Fallback Service**: Configured to use `ServeDir` as a fallback service for the root path.
- **Efficient & Reliable**: Built on the robust Tokio/Axum stack.

## Quick Start

### 1. Add Static Content
Ensure you have files in the `static/` directory (e.g., `static/index.html`).

### 2. Run the Server
The server listens on `http://127.0.0.1:8080`.

```bash
cargo run
```

### 3. Access Content
Open your browser or use `curl`:
```bash
curl http://localhost:8080/index.html
```

## Project Structure

- `src/main.rs`: Server configuration using `fallback_service` with `ServeDir`.
- `static/`: Directory containing static files to be served.
- `Cargo.toml`: Dependencies including `tower-http` with the `fs` feature.
