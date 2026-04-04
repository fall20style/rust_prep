#!/bin/bash

curl -s -X POST http://localhost:8080/calc \
  -H "Content-Type: application/json" \
  -d '{"a": 10, "b": 25}'
