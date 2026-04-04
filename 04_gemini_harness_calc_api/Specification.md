
## Calc REST API: Behavioral Specification

### Requirements
- Platform: Rust with Axum framework.
- Standard Operations:
    - **Add**: `POST /add`
    - **Subtract**: `POST /subtract`
    - **Multiply**: `POST /multiply`
    - **Divide**: `POST /divide` (must handle division by zero errors).

### Verification Mandate
All endpoints must be verified using the **Harness Engineering** approach defined in `AGENT.md`. This includes static evaluation of Axum extractor patterns and dynamic verification of HTTP status codes and JSON payloads.

## Unit Test Expectations
- Comprehensive coverage for all math operations.
- Boundary condition validation (e.g., division by zero).
- Use of the Axum testing utilities (e.g., `tower::ServiceExt`).
