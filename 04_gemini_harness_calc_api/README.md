# Rust Axum Calc API: Harness Engineering Approach

This project demonstrates a high-reliability development pattern using **Harness Engineering**. Following the logic of advanced evaluation frameworks (similar to Anthropic's methodology), we use automated static evaluation and dynamic verification to ensure production-grade API code.

### Core Architecture
1. **Specification.md**: Defines the behavioral and structural requirements.
2. **AGENT.md**: Outlines the phased workflow (Generator, Static Evaluator, and Dynamic Verifier).
3. **Execution Harness**: Orchestrates the automated synthesis and verification cycles.

### Getting Started
- Read `Specification.md` for API requirements.
- Review `AGENT.md` for the development lifecycle.
- Inspect `REPORT.md` for the latest harness evaluation results.

### Testing & Verification
The project includes a multi-layered verification harness:
1. **Unit Tests**: Run `cargo test` to execute internal logic and router tests.
2. **Integration Tests**: 
   - Start the server: `cargo run`
   - Run the E2E harness: `./test_api.sh`
