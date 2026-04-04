#!/bin/bash
# 01_axum_test_demo/harness_negative_calc.sh
# Harness: Verify the /calc endpoint correctly handles negative integers.

set -e

echo "--- Harness Engineering: Starting Black-Box Validation ---"

# 1. Build the project
cd 01_axum_test_demo
cargo build --quiet

# 2. Start the server in the background
cargo run --quiet &
SERVER_PID=$!
cd ..

# Ensure the server is killed even if the script fails
trap "kill $SERVER_PID" EXIT

# 3. Wait for the server to be ready
sleep 2

# 4. Define the test case (The "Harness" logic)
# Input: a=-50, b=-25 | Expected Result: -75
echo "Testing /calc with a=-50, b=-25..."
RESPONSE=$(curl -s -X POST http://localhost:8080/calc \
  -H "Content-Type: application/json" \
  -d '{"a": -50, "b": -25}')

# 5. Assert the result
RESULT=$(echo $RESPONSE | jq '.result')

if [ "$RESULT" -eq -75 ]; then
  echo "SUCCESS: Harness passed. Result is $RESULT."
else
  echo "FAILURE: Harness failed. Expected -75, got $RESULT."
  exit 1
fi

echo "--- Harness Validation Complete ---"
