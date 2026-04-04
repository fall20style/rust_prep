#!/bin/bash
set -e

echo "Starting E2E API Verification..."

# Check if server is running on 8080
if ! curl -s --head http://127.0.0.1:8080 | grep "200 OK" > /dev/null; then
  echo "[FAIL] Server is not responding on http://127.0.0.1:8080"
  exit 1
fi

# Check for index.html
if curl -s http://127.0.0.1:8080 | grep -q "xterm"; then
  echo "[PASS] UI with xterm.js is being served"
else
  echo "[FAIL] UI with xterm.js not found in response"
  exit 1
fi

# Check for /ws endpoint upgrade support
WS_UPGRADE=$(curl -i -s -N -H "Connection: Upgrade" \
  -H "Upgrade: websocket" \
  -H "Sec-WebSocket-Key: SGVsbG8sIHdvcmxkIQ==" \
  -H "Sec-WebSocket-Version: 13" \
  http://127.0.0.1:8080/ws | head -n 1)

if echo "$WS_UPGRADE" | grep -q "101 Switching Protocols"; then
  echo "[PASS] /ws endpoint correctly handles upgrade to WebSocket"
else
  echo "[FAIL] /ws endpoint failed to handle WebSocket upgrade. Response: $WS_UPGRADE"
  exit 1
fi

echo "E2E API Verification COMPLETED: ALL PASS"
