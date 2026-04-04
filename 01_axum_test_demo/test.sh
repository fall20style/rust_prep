#!/bin/bash
curl -X POST http://localhost:8080/echo \
     -H "Content-Type: application/json" \
     -d '{"message": "hello axum"}' | jq -s
