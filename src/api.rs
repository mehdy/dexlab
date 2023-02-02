use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct Meta {
    pub kind: String,
    pub api_version: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct Metadata {
    pub name: String,
    pub labels: HashMap<String, String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct Store<T> {
    pub metadata: Metadata,
    pub spec: T,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct Service<T> {
    pub metadata: Metadata,
    pub spec: T,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct Task {
    pub metadata: Metadata,
    pub spec: TaskSpec,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct TaskSpec {
    pub event_selector: EventSelector,
    pub service: String,
    pub handler: String,
    pub store: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct Event<T> {
    pub source: String,
    pub payload: T,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct EventSelector {
    pub match_labels: HashMap<String, String>,
    pub match_status: Vec<String>,
}
