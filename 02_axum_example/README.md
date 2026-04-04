# Axum Basic Example

A fundamental demonstration of the [Axum](https://github.com/tokio-rs/axum) web framework, showing basic GET and POST request handling with JSON data.

## Features

- **GET Endpoint**: Returns a simple text response.
- **POST Endpoint**: Receives a JSON payload and returns a serialized JSON response.
- **Asynchronous Execution**: Powered by `Tokio`.
- **JSON Support**: Uses `Serde` for seamless serialization and deserialization.

## API Endpoints

| Method | Endpoint | Description |
| :--- | :--- | :--- |
| `GET` | `/` | Returns "Hello, Axum GET Request!". |
| `POST` | `/users` | Accepts a `username` and returns a `User` object with a fixed ID. |

## Quick Start

### 1. Run the Server
The server starts on `http://127.0.0.1:8080`.

```bash
cargo run
```

### 2. Test the Endpoints

**GET Test:**
```bash
curl http://localhost:8080/
```

**POST Test:**
```bash
curl -X POST http://localhost:8080/users \
     -H "Content-Type: application/json" \
     -d '{"username": "rust_dev"}'
```

## Project Structure

- `src/main.rs`: Contains the application router and handler functions.
- `Cargo.toml`: Project dependencies (Axum, Serde, Tokio).
