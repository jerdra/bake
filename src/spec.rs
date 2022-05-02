use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::convert::TryFrom;
use std::ops::Deref;

#[derive(Debug, Deserialize, Serialize)]
pub struct FlourMap(pub HashMap<String, f32>);
impl Deref for FlourMap {
    type Target = HashMap<String, f32>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Deserialize, Debug)]
#[serde(try_from = "RawDoughSpec")]
pub struct DoughSpec {
    pub name: String,
    pub flour: FlourMap,
    pub hydration: f32,
    pub salt: f32,
    pub yeast: Option<f32>,
    pub description: Option<String>,
    pub starter: Option<f32>,
    pub mixins: Option<HashMap<String, f32>>,
}

/// Validation on dough specification
#[derive(Deserialize)]
pub struct RawDoughSpec {
    pub name: String,
    pub flour: FlourMap,
    pub hydration: f32,
    pub salt: f32,
    pub yeast: Option<f32>,
    pub description: Option<String>,
    pub starter: Option<f32>,
    pub mixins: Option<HashMap<String, f32>>,
}

impl TryFrom<RawDoughSpec> for DoughSpec {
    type Error = &'static str;
    fn try_from(value: RawDoughSpec) -> Result<Self, Self::Error> {
        let flour = make_percent(value.flour.0);
        let mixins = value.mixins.map(make_percent);
        let hydration = value.hydration / 100.0;
        let salt = value.salt / 100.0;
        let starter = value.starter.map(|starter| starter / 100.0);
        let yeast = value.yeast.map(|yeast| yeast / 100.0);

        Ok(DoughSpec {
            name: value.name,
            flour: FlourMap(flour),
            hydration,
            yeast,
            salt,
            description: value.description,
            starter,
            mixins,
        })
    }
}

#[derive(Deserialize, Debug)]
#[serde(try_from = "RawStarterSpec")]
pub struct StarterSpec {
    pub flour: FlourMap,
    pub hydration: f32,
}

/// Validation on starter specification
#[derive(Deserialize)]
pub struct RawStarterSpec {
    pub flour: FlourMap,
    pub hydration: f32,
}

impl TryFrom<RawStarterSpec> for StarterSpec {
    type Error = &'static str;
    fn try_from(value: RawStarterSpec) -> Result<Self, Self::Error> {
        let RawStarterSpec { flour, hydration } = value;
        let flour = make_percent(flour.0);
        let hydration = hydration / 100.0;
        Ok(StarterSpec {
            flour: FlourMap(flour),
            hydration,
        })
    }
}

fn make_percent(mapping: HashMap<String, f32>) -> HashMap<String, f32> {
    HashMap::from_iter(mapping.into_iter().map(|(key, value)| (key, value / 100.0)))
}
