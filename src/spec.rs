use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::ops::Deref;

#[derive(Debug, Deserialize, Serialize)]
pub struct FlourMap(HashMap<String, f32>);
impl Deref for FlourMap{
    type Target = HashMap<String, f32>;
    fn deref(&self) -> &Self::Target{
        &self.0
    }
}


/// Validation on dough specification
#[derive(Serialize, Deserialize, Debug)]
pub struct DoughSpec {
    pub name: String,
    pub flour: FlourMap,
    pub hydration: f32,
    pub salt: f32,
    pub description: Option<String>,
    pub starter: Option<f32>,
    pub extras: Option<HashMap<String, f32>>,
}

/// Validation on starter specification
#[derive(Serialize, Deserialize, Debug)]
pub struct StarterSpec {
    pub flour: FlourMap,
    pub hydration: f32,
}
