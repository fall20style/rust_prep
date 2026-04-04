#!/bin/bash
set -e

echo "Starting E2E API Verification..."

# Test /api/execute with 'ls'
RESPONSE=$(curl -s -X POST http://127.0.0.1:8080/api/execute \
  -H "Content-Type: application/json" \
  -d '{"command": "ls"}')

if echo "$RESPONSE" | grep -q "output"; then
  echo "[PASS] /api/execute 'ls' successful"
else
  echo "[FAIL] /api/execute 'ls' failed"
  echo "Response: $RESPONSE"
  exit 1
fi

# Test /api/execute with an error command
RESPONSE_ERR=$(curl -s -X POST http://127.0.0.1:8080/api/execute \
  -H "Content-Type: application/json" \
  -d '{"command": "command_that_does_not_exist"}')

if echo "$RESPONSE_ERR" | grep -q "error"; then
  echo "[PASS] /api/execute error handling successful"
else
  echo "[FAIL] /api/execute error handling failed"
  echo "Response: $RESPONSE_ERR"
  exit 1
fi

echo "E2E API Verification COMPLETED: ALL PASS"
