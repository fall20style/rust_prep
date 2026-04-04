# Rust Axum Web App: Axum Bash CLI

This project implements a web-based terminal interface that allows executing bash commands directly from a browser using **xterm.js** and **Axum**. It follows a strict **Harness Engineering** approach for high reliability.

### Features
- **Web Terminal**: Real-time terminal interface at `http://127.0.0.1:8080`.
- **Async Execution**: Non-blocking command execution using `tokio::process::Command`.
- **API Endpoint**: RESTful JSON API for remote command execution.
- **Harness Verified**: Fully tested via static evaluation and dynamic E2E verification.

### Getting Started
1. **Build and Run**:
   ```bash
   cargo run
   ```
2. **Access UI**: Open `http://127.0.0.1:8080` in your browser.
3. **Run Tests**:
   - Unit Tests: `cargo test`
   - E2E Verification: `./test_api.sh` (requires server running)

### API Documentation
#### POST `/api/execute`
Executes a bash command and returns the output.

**Request Body:**
```json
{
  "command": "ls -alh"
}
```

**Response Body:**
```json
{
  "output": "total 40K\ndrwxr-xr-x ...",
  "error": null
}
```

### Core Architecture
- `Specification.md`: Behavioral and structural requirements.
- `AGENT.md`: Phased workflow (Generator, Evaluator, Verifier).
- `REPORT.md`: Latest harness evaluation results (Status: 100/100).
