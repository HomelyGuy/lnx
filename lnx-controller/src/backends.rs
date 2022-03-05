use serde::{Serialize, Deserialize};

use scylladb_backend::{ConnectionConfig, ReplicationInfo};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BackendSelector {
    Scylla(ConnectionConfig)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IndexStorageConfig {
    Scylla(serde_json::Value),
}