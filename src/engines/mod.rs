pub mod document;
pub mod vector;
pub mod graph;
pub mod timeseries;
pub mod column;
pub mod keyvalue;
pub mod geospatial;
pub mod object;
pub mod records;

use std::sync::Arc;
use serde::{Deserialize, Serialize};

pub use document::DocumentEngine;
pub use vector::VectorEngine;
#[allow(unused_imports)]
pub use vector::VectorSearchResult;
pub use graph::GraphEngine;
#[allow(unused_imports)]
pub use graph::GraphEdge;
pub use timeseries::TimeSeriesEngine;
pub use column::ColumnEngine;
pub use keyvalue::KeyValueEngine;
pub use geospatial::GeospatialEngine;
#[allow(unused_imports)]
pub use geospatial::GeoLocationResult;
pub use object::ObjectEngine;
pub use records::RecordsEngine;

use crate::cluster::RaftClusterManager;
use crate::storage::{IdGenerator, LsmBTreeHybrid};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EngineInfo {
    pub name: String,
    pub description: String,
    pub key_format: String,
    pub active: bool,
}

pub struct EngineRegistry {
    pub document: Arc<DocumentEngine>,
    pub vector: Arc<VectorEngine>,
    pub graph: Arc<GraphEngine>,
    pub timeseries: Arc<TimeSeriesEngine>,
    pub column: Arc<ColumnEngine>,
    pub keyvalue: Arc<KeyValueEngine>,
    pub geospatial: Arc<GeospatialEngine>,
    pub object: Arc<ObjectEngine>,
    pub records: Arc<RecordsEngine>,
}

impl EngineRegistry {
    pub fn new(
        storage: Arc<LsmBTreeHybrid>,
        raft: Arc<RaftClusterManager>,
        id_gen: Arc<IdGenerator>,
    ) -> Self {
        Self {
            document: Arc::new(DocumentEngine::new(storage.clone(), raft.clone(), id_gen)),
            vector: Arc::new(VectorEngine::new(storage.clone(), raft.clone())),
            graph: Arc::new(GraphEngine::new(storage.clone(), raft.clone())),
            timeseries: Arc::new(TimeSeriesEngine::new(storage.clone(), raft.clone())),
            column: Arc::new(ColumnEngine::new(storage.clone(), raft.clone())),
            keyvalue: Arc::new(KeyValueEngine::new(storage.clone(), raft.clone())),
            geospatial: Arc::new(GeospatialEngine::new(storage.clone(), raft.clone())),
            object: Arc::new(ObjectEngine::new(storage.clone(), raft.clone())),
            records: Arc::new(RecordsEngine::new(storage, raft)),
        }
    }

    pub fn list_engines(&self) -> Vec<EngineInfo> {
        vec![
            EngineInfo {
                name: "DOCUMENT".to_string(),
                description: "JSON / NoSQL Document Store with Multi-Mode IDs & History".to_string(),
                key_format: "doc:{collection}:{id}".to_string(),
                active: true,
            },
            EngineInfo {
                name: "VECTOR".to_string(),
                description: "AI Vector Embeddings, Cosine Similarity & ANN Search".to_string(),
                key_format: "vec:{collection}:{id}".to_string(),
                active: true,
            },
            EngineInfo {
                name: "GRAPH".to_string(),
                description: "Labeled Property Graph (LPG) Vertices, Edges & Deep Traversal".to_string(),
                key_format: "graph:{graphId}:node:{id}".to_string(),
                active: true,
            },
            EngineInfo {
                name: "TIMESERIES".to_string(),
                description: "High-Frequency IoT Telemetry, Metrics & Range Aggregations".to_string(),
                key_format: "ts:{measurement}:{timestamp}".to_string(),
                active: true,
            },
            EngineInfo {
                name: "COLUMN".to_string(),
                description: "OLAP Columnar Projections & Analytical Families".to_string(),
                key_format: "col:{family}:{rowKey}".to_string(),
                active: true,
            },
            EngineInfo {
                name: "KEYVALUE".to_string(),
                description: "Atomic In-Memory MemTable Cache & Fast Raw KV Storage".to_string(),
                key_format: "kv:{namespace}:{key}".to_string(),
                active: true,
            },
            EngineInfo {
                name: "GEOSPATIAL".to_string(),
                description: "2D GIS Coordinates & Haversine Distance Proximity Search".to_string(),
                key_format: "geo:{collection}:{locId}".to_string(),
                active: true,
            },
            EngineInfo {
                name: "OBJECT".to_string(),
                description: "Chunked Binary BLOBs, Class Metadata & Media Streams".to_string(),
                key_format: "obj:{collection}:{id}".to_string(),
                active: true,
            },
            EngineInfo {
                name: "RECORDS".to_string(),
                description: "Strongly Typed Records, Component Validation & Schema Reflection".to_string(),
                key_format: "rec:{collection}:{id}".to_string(),
                active: true,
            },
        ]
    }
}

