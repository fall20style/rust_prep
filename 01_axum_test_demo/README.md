# Axum Test Demo

A simple web server demonstration using the [Axum](https://github.com/tokio-rs/axum) framework in Rust. This project showcases basic routing, JSON handling, and integration testing for web services.

## Features

- **Asynchronous Web Server**: Built with `Tokio` and `Axum`.
- **JSON Serialization/Deserialization**: Powered by `Serde`.
- **Integrated Testing**: Demonstrates how to test Axum routes without starting a full network server using `tower::ServiceExt::oneshot`.

## API Endpoints

| Method | Endpoint | Description |
| :--- | :--- | :--- |
| `GET` | `/user` | Returns a static user JSON object. |
| `POST` | `/echo` | Echoes back the JSON payload received in the request. |
| `POST` | `/calc` | Performs addition of two integers (`a` and `b`) provided in JSON. |

## Quick Start

### 1. Run the Server
The server runs on `http://127.0.0.1:8080`.

```bash
cargo run
```

### 2. Test the Endpoints
You can use `curl` to test the endpoints while the server is running:

**Echo Test:**
```bash
curl -X POST http://localhost:8080/echo \
     -H "Content-Type: application/json" \
     -d '{"message": "hello axum"}'
```

**Calculation Test:**
```bash
curl -X POST http://localhost:8080/calc \
     -H "Content-Type: application/json" \
     -d '{"a": 10, "b": 20}'
```

## Running Tests

This project includes automated tests to verify endpoint behavior and error handling.

```bash
cargo test
```

The tests cover:
- Successful retrieval of the `/user` data.
- Correct calculation in the `/calc` endpoint.
- Validation of invalid input types (e.g., passing a string where an integer is expected).

## Project Structure

- `src/main.rs`: Contains the application logic, router configuration, and test suite.
- `Cargo.toml`: Project dependencies and configuration.
- `test.sh`: A simple shell script to verify the `/echo` endpoint using `curl`.
