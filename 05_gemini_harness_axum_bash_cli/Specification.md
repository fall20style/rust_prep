## Axum web app: Behavioral Specification

### Requirements
- Platform: Rust with Axum framework.
- web server gets requests like 'ls -alh' in string. then make response to the client with the result.
  - 'ls -alh'
  - 12.7.0.0.1:8080 should show a UI with xterm.js to see the result like linux terminal.

### Verification Mandate
All endpoints must be verified using the **Harness Engineering** approach defined in `AGENT.md`. This includes static evaluation of Axum extractor patterns and dynamic verification of HTTP status codes and JSON payloads.

## Unit Test Expectations
- Comprehensive coverage for all math operations.
- Boundary condition validation (e.g., division by zero).
- Use of the Axum testing utilities (e.g., `tower::ServiceExt`).
