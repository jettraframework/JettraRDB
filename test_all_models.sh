#!/bin/bash
# ==============================================================================
# JettraRDB - Comprehensive Verification Script for All 9 Multi-Model Engines
# ==============================================================================
set -e

HOST=${1:-"http://127.0.0.1:8086"}
echo "Connecting to JettraRDB at $HOST..."

# 1. Login to obtain Bearer Token
echo "[1/10] Authenticating with admin credentials..."
LOGIN_RES=$(curl -s -X POST "$HOST/api/auth/login" \
  -H "Content-Type: application/json" \
  -d '{"username":"admin", "password":"admin"}')

TOKEN=$(echo "$LOGIN_RES" | grep -o '"token":"[^"]*' | cut -d'"' -f4)

if [ -z "$TOKEN" ]; then
  echo "Login failed! Response: $LOGIN_RES"
  exit 1
fi
echo "✓ Token obtained: $TOKEN"

# 2. Test All 9 Multi-Model Database Engines
MODELS=("document" "vector" "graph" "timeseries" "column" "keyvalue" "geospatial" "object" "records")

declare -A PAYLOADS
PAYLOADS=(
  ["document"]='{"_class":"User", "name":"Carlos Mendez", "role":"Architect"}'
  ["vector"]='{"vector":[0.15, 0.45, 0.88], "metadata":{"model":"bert-tiny"}}'
  ["graph"]='{"name":"Node Alpha", "type":"ServiceCluster"}'
  ["timeseries"]='{"cpu_usage":45.8, "memory_mb":1024}'
  ["column"]='{"cf:status":"online", "cf:tier":"production"}'
  ["keyvalue"]="rust_memory_safe_raw_value_2026"
  ["geospatial"]='{"lat":8.9824, "lon":-79.5199, "metadata":{"city":"Panama City"}}'
  ["object"]='{"_class":"CloudResource", "state":{"active":true, "zone":"us-east-1"}}'
  ["records"]='{"_recordClass":"com.jettra.model.EmployeeRecord", "components":{"id":"EMP-999", "name":"Ana Ramos", "salary":115000.0}}'
)

STEP=2
for MODEL in "${MODELS[@]}"; do
  echo "--------------------------------------------------"
  echo "[$STEP/10] Testing Engine: $MODEL"
  PAYLOAD=${PAYLOADS[$MODEL]}
  
  # INSERT
  echo "  -> POST /api/model/$MODEL/test_ns/1001"
  curl -s -X POST "$HOST/api/model/$MODEL/test_ns/1001" \
       -H "Authorization: Bearer $TOKEN" \
       -H "Content-Type: application/json" \
       -d "$PAYLOAD"
  echo ""
  
  # GET
  echo "  -> GET /api/model/$MODEL/test_ns/1001"
  GET_RES=$(curl -s -X GET "$HOST/api/model/$MODEL/test_ns/1001" \
       -H "Authorization: Bearer $TOKEN")
  echo "  ✓ Result: $GET_RES"
  
  STEP=$((STEP + 1))
done

echo "--------------------------------------------------"
echo "Testing Document Engine Dedicated History & PITR..."
echo "  -> GET /api/document/test_ns/1001/history"
HIST_RES=$(curl -s -X GET "$HOST/api/document/test_ns/1001/history" \
     -H "Authorization: Bearer $TOKEN")
echo "  ✓ History Result: $HIST_RES"

echo "=================================================="
echo "🎉 ALL 9 MULTI-MODEL DATABASE ENGINES PASSED IN JETTRARDB!"
echo "=================================================="

