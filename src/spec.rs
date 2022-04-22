use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Validation on dough specification
#[derive(Serialize, Deserialize, Debug)]
pub struct DoughSpec {
    pub name: String,
    pub flour: HashMap<String, f32>,
    pub hydration: f32,
    pub salt: f32,
    pub description: Option<String>,
    pub starter: Option<f32>,
    pub extras: Option<HashMap<String, f32>>,
}

/// Validation on starter specification
#[derive(Serialize, Deserialize, Debug)]
pub struct StarterSpec {
    pub flour: HashMap<String, f32>,
    pub hydration: f32,
}
