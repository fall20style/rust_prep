# Harness Engineering with Gemini CLI

Yes, you can absolutely work in a **Harness Engineering** style with Gemini CLI. In fact, this approach is highly recommended as it aligns perfectly with the agent's core mandates for empirical reproduction and rigorous validation.

## What is Harness Engineering?

Harness Engineering is a development methodology where you prioritize building a **test harness** or **reproduction script** before writing any implementation code. This harness acts as the "source of truth" for the task at hand.

- **For Bug Fixes:** The harness is a script that reliably reproduces the bug (failing state).
- **For New Features:** The harness defines the expected behavior and interface (target state).

## Why Use This Style with Gemini CLI?

1.  **Reduced Ambiguity:** A harness provides a concrete, executable specification.
2.  **Empirical Proof:** It moves the conversation from "I think it works" to "The harness passes."
3.  **Context Efficiency:** Instead of reading hundreds of lines of code to verify a change, you (and the agent) can simply run the harness.
4.  **Safety:** It ensures that your changes actually solve the intended problem without side effects.

## The Workflow in Gemini CLI

You can direct the agent to follow these steps:

### 1. The "Research & Reproduce" Phase
Ask the agent to create a minimal reproduction script (e.g., `repro.sh` or a new test file).
> **User:** "Research the reported issue in the `/calc` endpoint and create a shell script harness that reproduces the 422 error when passing null values."

### 2. The "Baseline Verification" Phase
The agent runs the harness to confirm it fails as expected.
> **Agent:** (Runs `bash repro.sh`, sees failure, confirms the bug is real).

### 3. The "Strategic Implementation" Phase
The agent modifies the code to satisfy the harness.
> **Agent:** (Applies surgical fixes to `src/main.rs`).

### 4. The "Validation" Phase
The agent runs the harness again to verify the fix.
> **Agent:** (Runs `bash repro.sh`, sees success).

### 5. Integration
Once verified, the harness logic can be integrated into the permanent test suite (e.g., `cargo test`).

## Example: A Typical Harness Command

In a Rust project like this one, a harness might look like this:

```bash
# harness.sh
set -e
cargo build
# Start server in background
cargo run &
PID=$!
sleep 2

# Test the harness
curl -s -X POST http://localhost:8080/calc \
  -H "Content-Type: application/json" \
  -d '{"a": 10, "b": 20}' | grep '"result":30'

# Cleanup
kill $PID
```

## Tips for Best Results

- **Ask for the Harness First:** Explicitly tell the agent: "Before fixing the bug, write a reproduction script."
- **Keep it Minimal:** Harnesses should be fast and focused on one specific behavior.
- **Use `run_shell_command`:** This is your primary tool for executing harnesses and seeing immediate results.

By adopting this style, you turn Gemini CLI into a precision engineering tool that guarantees the integrity of your codebase.
