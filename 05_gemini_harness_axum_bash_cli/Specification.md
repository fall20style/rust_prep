## Axum web app: Behavioral Specification

### Requirements
- [x] Platform: Rust with Axum framework.
- [x] Web server gets requests like 'ls -alh' in string. then make response to the client with the result.
- [x] 'ls -alh' command execution support.
- [x] 12.7.0.0.1:8080 shows a UI with xterm.js to see the result like linux terminal.

- [x] Emulating terminal env in web browser should be supported.
- [x] List up all linux command that this system provides as a whitelist in order to make someone hidden for security.
- [x] I want vim or other terminal app to run on this web app.
- [x] tab or space key supported

### Verification Mandate
- [x] All endpoints verified using the **Harness Engineering** approach.
- [x] Static evaluation of Axum extractor patterns.
- [x] Dynamic verification of HTTP status codes and JSON payloads.

## Unit Test Expectations
- [x] Comprehensive coverage for command execution.
- [x] Boundary condition validation (non-existent commands).
- [x] Use of Axum testing utilities (`tower::ServiceExt`).
