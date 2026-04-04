#!/bin/bash

# Calculator API Integration Test Harness
# This script performs dynamic verification of the Axum REST API endpoints.

BASE_URL="http://localhost:3000"

echo "--------------------------------------------------"
echo "🚀 Starting API Verification Harness..."
echo "--------------------------------------------------"

# Function to perform a POST request and print results
test_endpoint() {
    local endpoint=$1
    local a=$2
    local b=$3
    local expected=$4
    local description=$5

    echo -n "Testing $description ($endpoint): "

    response=$(curl -s -X POST "$BASE_URL/$endpoint" \
        -H "Content-Type: application/json" \
        -d "{\"a\": $a, \"b\": $b}")

    # Extract result from JSON using grep/sed (to avoid dependency on jq for simple cases)
    result=$(echo "$response" | grep -oP '"result":\K[^,}]*')

    if [ "$result" == "$expected" ]; then
        echo "✅ PASS (Result: $result)"
    else
        echo "❌ FAIL (Expected: $expected, Got: $response)"
    fi
}

# 1. Test Addition
test_endpoint "add" 10 5 15 "Addition"

# 2. Test Subtraction
test_endpoint "subtract" 10 5 5 "Subtraction"

# 3. Test Multiplication
test_endpoint "multiply" 10 5 50 "Multiplication"

# 4. Test Division
test_endpoint "divide" 10 2 5 "Division"

# 5. Test Division by Zero (Expected: 400 Bad Request)
echo -n "Testing Division by Zero: "
http_code=$(curl -s -o /dev/null -w "%{http_code}" -X POST "$BASE_URL/divide" \
    -H "Content-Type: application/json" \
    -d '{"a": 10, "b": 0}')

if [ "$http_code" == "400" ]; then
    echo "✅ PASS (Correctly returned 400 Bad Request)"
else
    echo "❌ FAIL (Expected 400, Got $http_code)"
fi

echo "--------------------------------------------------"
echo "🏁 Verification Complete."
echo "--------------------------------------------------"
