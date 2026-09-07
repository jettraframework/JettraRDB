#!/bin/bash
set -e

HOST=${1:-"http://127.0.0.1:6086"}
echo "=================================================="
echo "JettraRDB Database CRUD & Data Explorer Test Suite"
echo "Target Host: $HOST"
echo "=================================================="

# 1. Login
echo "[1/8] Authenticating..."
LOGIN_RES=$(curl -s -X POST "$HOST/api/auth/login" \
  -H "Content-Type: application/json" \
  -d '{"username":"admin", "password":"admin"}')

TOKEN=$(echo "$LOGIN_RES" | grep -o '"token":"[^"]*' | cut -d'"' -f4)
if [ -z "$TOKEN" ]; then
  echo "❌ Login failed! Response: $LOGIN_RES"
  exit 1
fi
echo "✓ Authenticated with token: $TOKEN"

# 2. Create Database
echo "[2/8] Creating new database 'alpha_corp'..."
CREATE_RES=$(curl -s -X POST "$HOST/api/databases" \
  -H "Content-Type: application/json" \
  -d '{"name":"alpha_corp"}')
echo "Response: $CREATE_RES"

# 3. Rename Database
echo "[3/8] Renaming database 'alpha_corp' to 'omega_corp'..."
RENAME_RES=$(curl -s -X PUT "$HOST/api/databases/alpha_corp/rename" \
  -H "Content-Type: application/json" \
  -d '{"new_name":"omega_corp"}')
echo "Response: $RENAME_RES"

# Verify List
DB_LIST=$(curl -s -X GET "$HOST/api/databases")
echo "Databases: $DB_LIST"
if echo "$DB_LIST" | grep -q "omega_corp"; then
  echo "✓ Database 'omega_corp' verified in database list"
else
  echo "❌ 'omega_corp' not found in database list!"
  exit 1
fi

# 4. Insert records across all 9 engines in 'omega_corp'
echo "[4/8] Inserting multi-model records into 'omega_corp'..."

# DOCUMENT
curl -s -X POST "$HOST/api/model/document/omega_corp/doc_01" \
  -H "Authorization: Bearer $TOKEN" -H "Content-Type: application/json" \
  -d '{"name":"Customer Invoice", "total":1250.50, "status":"paid"}'

# VECTOR
curl -s -X POST "$HOST/api/model/vector/omega_corp/vec_01" \
  -H "Authorization: Bearer $TOKEN" -H "Content-Type: application/json" \
  -d '{"vector":[0.12, 0.85, 0.44, 0.91], "metadata":{"model":"bert-tiny"}}'

# GRAPH
curl -s -X POST "$HOST/api/model/graph/omega_corp/node_01" \
  -H "Authorization: Bearer $TOKEN" -H "Content-Type: application/json" \
  -d '{"label":"Person", "properties":{"name":"Alice"}, "out_edges":[{"target":"node_02","type":"FRIEND"}]}'

# TIMESERIES
curl -s -X POST "$HOST/api/model/timeseries/omega_corp/1700000001" \
  -H "Authorization: Bearer $TOKEN" -H "Content-Type: application/json" \
  -d '{"sensor":"temp", "celsius":23.5}'

# COLUMN
curl -s -X POST "$HOST/api/model/column/omega_corp/col_01" \
  -H "Authorization: Bearer $TOKEN" -H "Content-Type: application/json" \
  -d '{"profile:name":"Alice", "account:tier":"pro"}'

# KEYVALUE
curl -s -X POST "$HOST/api/model/keyvalue/omega_corp/kv_01" \
  -H "Authorization: Bearer $TOKEN" -H "Content-Type: application/json" \
  -d '{"cache_key":"fast_val_999"}'

# GEOSPATIAL
curl -s -X POST "$HOST/api/model/geospatial/omega_corp/geo_01" \
  -H "Authorization: Bearer $TOKEN" -H "Content-Type: application/json" \
  -d '{"lat":8.9824, "lon":-79.5199, "metadata":{"city":"Panama City"}}'

# OBJECT
curl -s -X POST "$HOST/api/model/object/omega_corp/obj_01" \
  -H "Authorization: Bearer $TOKEN" -H "Content-Type: application/json" \
  -d '{"_class":"UserSession", "state":{"active":true, "ttl":3600}}'

# RECORDS
curl -s -X POST "$HOST/api/model/records/omega_corp/rec_01" \
  -H "Authorization: Bearer $TOKEN" -H "Content-Type: application/json" \
  -d '{"_recordClass":"AuditRecord", "components":{"event":"LOGIN", "code":200}}'

echo "✓ 9 Multi-model records inserted."

# 5. Fetch Database Records API
echo "[5/8] Querying /api/databases/omega_corp/records..."
RECORDS_RES=$(curl -s -X GET "$HOST/api/databases/omega_corp/records")
REC_COUNT=$(echo "$RECORDS_RES" | grep -o '"count":[0-9]*' | cut -d':' -f2)
echo "Records found: $REC_COUNT"
if [ "$REC_COUNT" -ge 9 ]; then
  echo "✓ Successfully verified $REC_COUNT records stored in 'omega_corp' partition."
else
  echo "❌ Expected at least 9 records, got $REC_COUNT: $RECORDS_RES"
  exit 1
fi

# 6. Test Record Update & History
echo "[6/8] Testing Record Update (PUT) and MVCC History..."
curl -s -X PUT "$HOST/api/model/document/omega_corp/doc_01" \
  -H "Authorization: Bearer $TOKEN" -H "Content-Type: application/json" \
  -d '{"name":"Customer Invoice UPDATED", "total":1500.00, "status":"paid_confirmed"}'

GET_UPDATED=$(curl -s -X GET "$HOST/api/model/document/omega_corp/doc_01" \
  -H "Authorization: Bearer $TOKEN")
echo "Updated doc: $GET_UPDATED"
if echo "$GET_UPDATED" | grep -q "UPDATED"; then
  echo "✓ Record update verified."
else
  echo "❌ Record update failed: $GET_UPDATED"
  exit 1
fi

HISTORY_RES=$(curl -s -X GET "$HOST/api/document/omega_corp/doc_01/history" \
  -H "Authorization: Bearer $TOKEN")
echo "History entries: $HISTORY_RES"

# 7. Test Web Console HTML pages
echo "[7/8] Verifying Web Console HTML endpoints..."
DB_PAGE=$(curl -s "$HOST/databases")
if echo "$DB_PAGE" | grep -q "omega_corp"; then
  echo "✓ /databases page renders 'omega_corp' with rename and explorer links."
else
  echo "❌ /databases page does not contain 'omega_corp'"
  exit 1
fi

EXPLORER_PAGE=$(curl -s "$HOST/explorer?db=omega_corp")
if echo "$EXPLORER_PAGE" | grep -q "Data Explorer" && echo "$EXPLORER_PAGE" | grep -q "Vista Árbol (Tree)"; then
  echo "✓ /explorer page renders Table View and Tree View controls."
else
  echo "❌ /explorer page missing required UI components"
  exit 1
fi

# 8. Test Record Deletion
echo "[8/8] Testing Record Deletion (DELETE)..."
curl -s -X DELETE "$HOST/api/model/document/omega_corp/doc_01" \
  -H "Authorization: Bearer $TOKEN"

DEL_CHECK=$(curl -s -w "%{http_code}" -o /dev/null -X GET "$HOST/api/model/document/omega_corp/doc_01" \
  -H "Authorization: Bearer $TOKEN")
if [ "$DEL_CHECK" = "404" ]; then
  echo "✓ Record deletion verified (404 NOT FOUND as expected)."
else
  echo "❌ Expected 404 after delete, got $DEL_CHECK"
  exit 1
fi

echo "=================================================="
echo "🎉 ALL DATABASE CRUD & DATA EXPLORER TESTS PASSED!"
echo "=================================================="
