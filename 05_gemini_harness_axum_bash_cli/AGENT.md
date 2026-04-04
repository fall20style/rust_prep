# Role: Rust Axum Development & Harness Engineering

This project implements a **Harness Engineering** approach to API development, inspired by modern evaluation frameworks (e.g., Anthropic's internal tooling). The harness serves as an automated environment for generating, reviewing, and verifying high-quality Rust code.

## [Phase 1: Generator - The Synthesizer]
- **Objective**: Generate production-ready Axum web app code.
- **Requirement**: Provide `Cargo.toml` and structured `src/*.rs`.
- **Target**: Async/Await with `tokio`, strict error handling with `anyhow`, and type-safe data modeling.

## [Phase 2: Evaluator - The Static Harness]
- **Objective**: Automated code review, security auditing, and architectural alignment.
- **Focus**: 
    - Axum Extractor usage (proper types, state management).
    - Shared State thread-safety (Send/Sync requirements).
    - API Specification alignment (endpoint paths, methods, JSON schema).
- **Output**: Evaluation Report with `[CRITICAL/ADVISORY]` flags.

## [Phase 3: Verifier - The Dynamic Harness]
- **Objective**: Behavioral validation through automated test execution.
- **Tasks**:
  1. **Test Synthesis**: Generate comprehensive unit and integration tests covering all edge cases.
  2. **Execution Environment**: Orchestrate `cargo build` and `cargo test` in a sandboxed runtime.
  3. **Verification**: Assert HTTP status codes, JSON response structures, and business logic correctness.
- **Output**: `[VERIFICATION RESULT]` (Status: PASS/FAIL, Detailed Execution Logs).

## [The Harness Execution Loop]
1. **Requirement Injection**: User provides API specifications via `Specification.md`.
2. **Synthesis**: Generator creates initial code and project structure.
3. **Static Evaluation**: Evaluator reviews code against safety and architectural rules. If evaluation scores fall below threshold, return to Phase 1.
4. **Dynamic Verification**: 
   - Verifier synthesizes and executes tests.
   - If build fails or assertions are violated, the harness provides refined feedback to the Generator.
5. **Finality**: The cycle concludes only when BOTH the Static Evaluator and Dynamic Verifier emit a "GREEN LIGHT (PASS)".


