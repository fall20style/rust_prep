# Rust Prep

A repository for Rust learning, practice, and demonstration projects, focusing on the Axum web framework.

## Project List

- **[01 Axum Test Demo](./01_axum_test_demo/)**: A web server demonstrating Axum routing, JSON handling, and integration testing.
- **[02 Axum Example](./02_axum_example/)**: A basic Axum server showcasing GET and POST requests with JSON processing.
- **[03 Axum Web Server](./03_axum_websvr/)**: A static file server using `tower-http` to serve content from a local directory.
- **[04 Gemini Harness Calc API](./04_gemini_harness_calc_api/)**: A high-reliability API development demonstration using the "Harness Engineering" approach.
- **[05 Gemini Harness Axum Bash CLI](./05_gemini_harness_axum_bash_cli/)**: A web-based terminal interface using `xterm.js` and Axum, following the "Harness Engineering" approach for high reliability.
- **[06 Axum Bash CLI Dockerfile](./06_axum_bash_cli_Dockerfile/)**: A Dockerized version of a web-based terminal using Axum and `portable-pty`.
- **[07 Axum Bollard Term](./07_axum_bollard_term/)**: An advanced web terminal that connects to Docker containers using the `bollard` library and `xterm.js`.

## Documentation

- **[Harness Engineering](./Docs/HARNESS_ENGINEERING.md)**: A guide on using the Harness Engineering style with Gemini CLI for robust development.

## Recent Updates

- **CI/CD Integration**: Added GitHub Actions workflow for building and testing the `07_axum_bollard_term` project.
- **Dockerization**: Provided Dockerfiles for `06_axum_bash_cli_Dockerfile` and `07_axum_bollard_term` to support containerized deployments.
- **Bollard Integration**: Implemented container interaction using the `bollard` library for enhanced terminal capabilities.
