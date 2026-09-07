# JettraRDB - Comprehensive Usage Guide

This manual covers the complete usage of **JettraRDB**, including authentication, all 9 multi-model database operational engines, snapshot backups, and the integrated JettraFlux Web Management Console.

---

## Table of Contents
1. [Authentication & Authorization](#1-authentication--authorization)
2. [Document Engine (`DOCUMENT`)](#2-document-engine-document)
3. [Vector Engine (`VECTOR`)](#3-vector-engine-vector)
4. [Graph Engine (`GRAPH`)](#4-graph-engine-graph)
5. [TimeSeries Engine (`TIMESERIES`)](#5-timeseries-engine-timeseries)
6. [Column Engine (`COLUMN`)](#6-column-engine-column)
7. [KeyValue Engine (`KEYVALUE`)](#7-keyvalue-engine-keyvalue)
8. [Geospatial Engine (`GEOSPATIAL`)](#8-geospatial-engine-geospatial)
9. [Object Engine (`OBJECT`)](#9-object-engine-object)
10. [Records Engine (`RECORDS`)](#10-records-engine-records)
11. [Database Administration API](#11-database-administration-api)
12. [Backups & Disaster Recovery](#12-backups--disaster-recovery)
13. [JettraFlux Web Management Console](#13-jettraflux-web-management-console)

---

## 1. Authentication & Authorization

All API endpoints (except authentication and the Web UI) require an `Authorization: Bearer <TOKEN>` HTTP header.

### 1.1 Login & Obtain Token
```bash
curl -s -X POST http://localhost:8086/api/auth/login \
  -H "Content-Type: application/json" \
  -d '{
    "username": "admin",
    "password": "admin"
  }'
```

**Response:**
```json
{
  "token": "d2f47d4e-e1bb-45e3-9cf6-cbfa1e7fa890",
  "user": "admin",
  "requires_password_change": false
}
```

Store the token in an environment variable for subsequent commands:
```bash
export TOKEN="d2f47d4e-e1bb-45e3-9cf6-cbfa1e7fa890"
```

### 1.2 Change Password
```bash
curl -X POST http://localhost:8086/api/auth/change-password \
  -H "Content-Type: application/json" \
  -d '{
    "username": "super-user",
    "old_password": "superUserZ",
    "new_password": "MyNewSecurePassword2026!"
  }'
```

---

## 2. Document Engine (`DOCUMENT`)

Stores JSON/BSON document trees with multi-strategy identifier generation (`manual`, `autoincrement`, `uuid`), MVCC revision history, and Point-In-Time-Recovery (PITR).

### 2.1 Insert with Auto UUID Generation
```bash
curl -X POST http://localhost:8086/api/document/customers \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "Acme Global",
    "country": "Panama",
    "balance": 45000.0,
    "active": true
  }'
```

### 2.2 Insert with Auto-Incrementing Numeric ID
```bash
curl -X POST "http://localhost:8086/api/document/orders/next?id_mode=autoincrement" \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "orderId": "ORD-100",
    "total": 1250.75,
    "items": ["Laptop", "Mouse"]
  }'
```

### 2.3 Insert with Manual ID
```bash
curl -X POST http://localhost:8086/api/document/customers/CUST-001 \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "Carlos Gomez",
    "tier": "Gold"
  }'
```

### 2.4 Retrieve Document
```bash
curl -X GET http://localhost:8086/api/document/customers/CUST-001 \
  -H "Authorization: Bearer $TOKEN"
```

### 2.5 Document Revision History (MVCC)
```bash
curl -X GET http://localhost:8086/api/document/customers/CUST-001/history \
  -H "Authorization: Bearer $TOKEN"
```

**Response:**
```json
[
  {
    "version_number": 2,
    "timestamp": 1755735499000,
    "formatted_date": "2026-08-21 14:18:19",
    "payload": "{\"name\":\"Carlos Gomez\",\"tier\":\"Platinum\"}",
    "is_current": true
  },
  {
    "version_number": 1,
    "timestamp": 1755735492000,
    "formatted_date": "2026-08-21 14:18:12",
    "payload": "{\"name\":\"Carlos Gomez\",\"tier\":\"Gold\"}",
    "is_current": false
  }
]
```

### 2.6 Point-in-Time Restore (Rollback)
```bash
curl -X POST "http://localhost:8086/api/document/customers/CUST-001/restore?timestamp=1755735492000" \
  -H "Authorization: Bearer $TOKEN"
```

### 2.7 Delete Document
```bash
curl -X DELETE http://localhost:8086/api/document/customers/CUST-001 \
  -H "Authorization: Bearer $TOKEN"
```

---

## 3. Vector Engine (`VECTOR`)

Specialized for AI embeddings, Cosine similarity, and Nearest Neighbor (ANN) search.

### 3.1 Store Vector Embedding
```bash
curl -X POST http://localhost:8086/api/model/vector/articles/art-01 \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "vector": [0.12, 0.94, -0.33, 0.45, 0.78],
    "metadata": {
      "title": "Introduction to Rust Async Storage",
      "author": "Jettra Team"
    }
  }'
```

### 3.2 Retrieve Vector Document
```bash
curl -X GET http://localhost:8086/api/model/vector/articles/art-01 \
  -H "Authorization: Bearer $TOKEN"
```

---

## 4. Graph Engine (`GRAPH`)

Labeled Property Graph (LPG) engine with vertices, directed labeled edges, and adjacency queries.

### 4.1 Create Vertex (Node)
```bash
curl -X POST http://localhost:8086/api/model/graph/social/user_1 \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "Alice Cooper",
    "department": "Engineering"
  }'
```

### 4.2 Retrieve Vertex
```bash
curl -X GET http://localhost:8086/api/model/graph/social/user_1 \
  -H "Authorization: Bearer $TOKEN"
```

### 4.3 Delete Vertex
```bash
curl -X DELETE http://localhost:8086/api/model/graph/social/user_1 \
  -H "Authorization: Bearer $TOKEN"
```

---

## 5. TimeSeries Engine (`TIMESERIES`)

High-frequency IoT metric and telemetry engine ordered by timestamp epoch.

### 5.1 Ingest Telemetry Metric
```bash
# Ingest reading with microsecond timestamp: 1755735492000
curl -X POST http://localhost:8086/api/model/timeseries/temperature_sensor/1755735492000 \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "celsius": 24.5,
    "humidity": 62.0,
    "battery": 98.4
  }'
```

### 5.2 Retrieve Telemetry Point
```bash
curl -X GET http://localhost:8086/api/model/timeseries/temperature_sensor/1755735492000 \
  -H "Authorization: Bearer $TOKEN"
```

---

## 6. Column Engine (`COLUMN`)

Optimized for analytical queries (OLAP) and column families.

### 6.1 Insert Row with Sparse Columns
```bash
curl -X POST http://localhost:8086/api/model/column/analytics/session_992 \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "cf_user:id": "USR-102",
    "cf_metrics:duration_sec": 412,
    "cf_metrics:bounce": false
  }'
```

### 6.2 Retrieve Row
```bash
curl -X GET http://localhost:8086/api/model/column/analytics/session_992 \
  -H "Authorization: Bearer $TOKEN"
```

---

## 7. KeyValue Engine (`KEYVALUE`)

Raw atomic in-memory MemTable key-value store for ultra-low latency caching.

### 7.1 Put Key-Value
```bash
curl -X POST http://localhost:8086/api/model/keyvalue/session_cache/sess_token_abc \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: text/plain" \
  -d "user_id=8841;authenticated=true;role=admin"
```

### 7.2 Get Key-Value
```bash
curl -X GET http://localhost:8086/api/model/keyvalue/session_cache/sess_token_abc \
  -H "Authorization: Bearer $TOKEN"
```

### 7.3 Delete Key-Value
```bash
curl -X DELETE http://localhost:8086/api/model/keyvalue/session_cache/sess_token_abc \
  -H "Authorization: Bearer $TOKEN"
```

---

## 8. Geospatial Engine (`GEOSPATIAL`)

Stores 2D GIS coordinates (latitude and longitude) with Haversine distance proximity queries.

### 8.1 Ingest Location
```bash
curl -X POST http://localhost:8086/api/model/geospatial/warehouses/wh_central \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "lat": 8.9824,
    "lon": -79.5199,
    "metadata": {
      "city": "Panama City",
      "capacity_sqm": 12000
    }
  }'
```

### 8.2 Get Location
```bash
curl -X GET http://localhost:8086/api/model/geospatial/warehouses/wh_central \
  -H "Authorization: Bearer $TOKEN"
```

---

## 9. Records Engine (`RECORDS`)

Strongly typed schema validation, component introspection, and selective field projection.

### 9.1 Store Structured Record
```bash
curl -X POST http://localhost:8086/api/model/records/employees/EMP-001 \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "_recordClass": "com.enterprise.model.EmployeeRecord",
    "_schema": {
      "id": "String",
      "fullName": "String",
      "department": "String",
      "salary": "Double",
      "active": "Boolean"
    },
    "components": {
      "id": "EMP-001",
      "fullName": "Carlos Mendez",
      "department": "Engineering",
      "salary": 95000.0,
      "active": true
    }
  }'
```

### 9.2 Retrieve Full Record
```bash
curl -X GET http://localhost:8086/api/model/records/employees/EMP-001 \
  -H "Authorization: Bearer $TOKEN"
```

### 9.3 Field Projection Query (`?fields=a,b`)
Retrieve only selected fields without network payload bloat:
```bash
curl -X GET "http://localhost:8086/api/model/records/employees/EMP-001?fields=fullName,salary" \
  -H "Authorization: Bearer $TOKEN"
```

**Response:**
```json
{
  "fullName": "Carlos Mendez",
  "salary": 95000.0
}
```

---

## 11. Database Administration API
## 11. Database Administration & Records API

### 11.1 List All Dedicated Databases
JettraRDB supports dynamic per-database isolated partition management over the hybrid LSM-Tree + B-Tree storage engine.

### 11.1 Create a New Database
```bash
curl -X POST http://localhost:8086/api/databases \
  -H "Content-Type: application/json" \
  -d '{"name": "finance_prod"}'
```

**Response (HTTP 201 CREATED):**
```json
{
  "database": "finance_prod",
  "status": "success"
}
```

### 11.2 Rename a Database
Seamlessly renames the active partition on disk and migrates internal partition state in zero downtime:
```bash
curl -X PUT http://localhost:8086/api/databases/finance_prod/rename \
  -H "Content-Type: application/json" \
  -d '{"new_name": "finance_prod_v2"}'
```

**Response (HTTP 200 OK):**
```json
{
  "status": "success",
  "old_name": "finance_prod",
  "new_name": "finance_prod_v2"
}
```

### 11.3 List All Dedicated Databases
```bash
curl -X GET http://localhost:8086/api/databases
```

### 11.2 Drop an Isolated Database
**Response (HTTP 200 OK):**
```json
{
  "databases": ["finance_prod_v2", "ecommerce", "analytics"]
}
```

### 11.4 Get All Records in a Database Partition
Returns all records across all 9 engines stored within the specified database partition, including engine classification, keys, namespaces, payload snippets, and MVCC version counts:
```bash
curl -X GET http://localhost:8086/api/databases/finance_prod_v2/records
```

**Response (HTTP 200 OK):**
```json
{
  "database": "finance_prod_v2",
  "count": 1,
  "records": [
    {
      "key": "doc:invoices:INV-1001",
      "engine": "DOCUMENT",
      "namespace": "invoices",
      "id": "INV-1001",
      "payload": "{\"total\": 1250.50, \"status\": \"paid\"}",
      "version_count": 2,
      "timestamp": 1788798887598
    }
  ]
}
```

### 11.5 Drop an Isolated Database
Permanently purges the on-disk partition directory and closes journal files (`_system` database is protected):
```bash
curl -X DELETE http://localhost:8086/api/databases/old_test_db
```

---

## 12. Backups & Disaster Recovery

### 12.1 Create Instant Hot Backup
```bash
curl -X POST http://localhost:8086/api/backup
```

**Response:**
```json
{
  "status": "Backup created successfully",
  "path": "./jettra_backup_20260907_123045"
}
```

---

## 13. JettraFlux Web Management Console

JettraRDB embeds the complete JettraFlux Web UI directly inside the binary. Access through any browser at `http://localhost:50050`:
JettraRDB embeds the complete JettraFlux Web UI directly inside the binary. Access through any browser at `http://localhost:50050` (or `http://localhost:8086`):

- **`/` or `/dashboard`**: Real-time KPI summary cards, throughput, 9 engines layout.
- **`/databases`**: Interactive visual database explorer and drop actions.
- **`/engines`**: Interactive 9 engines workbench.
- **`/` or `/dashboard`**: Real-time KPI summary cards, active databases counter, memory latency (0ms Zero GC), 9 multi-model engines layout.
- **`/databases`**: Visual database administration:
  - **"Nueva Base de Datos"**: Interactive modal to create isolated database partitions on demand.
  - **"Renombrar"**: In-place rename modal for live database partition migration.
  - **"Explorar"**: 1-click navigation to explore records in the Data Explorer.
  - **"Drop"**: Safe deletion of partition storage.
- **`/explorer` (Data Explorer)**:
  - **Database Selector**: Switch instantly between databases or filter by active database (`/explorer?db=NAME`).
  - **Search & Filter Bar**: Real-time search across keys, namespaces, IDs, and payload contents.
  - **Engine Pills**: Quick filters for all 9 engines (`DOCUMENT`, `VECTOR`, `GRAPH`, `TIMESERIES`, `COLUMN`, `KEYVALUE`, `GEOSPATIAL`, `OBJECT`, `RECORDS`, `TODOS`).
  - **📊 Vista Tabla (Table View)**:
    - Sortable rows showing Engine badge, Key, Namespace, ID, Payload snippet, MVCC version counter.
    - Quick actions per row: `[👁️ Ver]`, `[✏️ Editar]`, `[🕒 Historial]`, `[🗑️ Eliminar]`.
  - **🌳 Vista Árbol (Hierarchical Tree View)**:
    - Collapsible multi-tier DOM tree: `Database -> Engine -> Namespace -> Record ID -> JSON Properties`.
    - Recursively formats JSON data structures (strings, numbers, booleans, objects, arrays) with syntax color coding.
    - Includes `[Expandir Todo]` and `[Colapsar Todo]` global toggles.
  - **Interactive Multi-Model CRUD Modals**:
    - **Crear Registro**: Pre-loads specialized templates tailored for each of the 9 engines on dropdown selection.
    - **Ver Registro**: Prettified JSON code viewer with 1-click clipboard copy.
    - **Editar Registro**: Payload editor with instant `PUT /api/model/...` synchronization.
    - **Historial & Rollback (MVCC)**: Inspect all historical versions with timestamps and 1-click rollback.
    - **Eliminar Registro**: Confirmation prompt and atomic deletion.
- **`/engines`**: Interactive 9 engines workbench with live code examples.
- **`/users`**: RBAC permissions and user profiles.
- **`/components`**: Raft cluster topology, nodes health, and replication logs.
- **`/information`**: Architectural whitepaper and specifications.
- **`/swagger-ui`**: Embedded OpenAPI explorer for point-and-click testing.

