use serde::Deserialize;
use std::collections::HashMap;
use std::convert::TryFrom;

/// Mapping from String to Percentage value
type IngredientMap = HashMap<String, BakersPercent>;

/// Percentage value
#[derive(Debug)]
pub struct Percent(f32);
impl TryFrom<f32> for Percent {
    type Error = &'static str;
    fn try_from(value: f32) -> Result<Self, Self::Error> {
        if !(0.0..=100.0).contains(&value) {
            Err("Value must be a percentage between 0 and 100!")
        } else {
            Ok(Percent(value))
        }
    }
}

/// A ingredient in Baker's percentages
#[derive(Debug)]
pub struct BakersPercent {
    name: String,
    percent: f32,
}

#[derive(Deserialize, Debug)]
#[serde(try_from = "RawDoughSpec")]
pub struct DoughSpec {
    pub name: String,
    pub description: Option<String>,
    pub flour: IngredientMap,
    pub hydration: IngredientMap,
    pub ingredients: Vec<BakersPercent>,
    pub starter: Option<BakersPercent>,
}

/// Validation on dough specification
#[derive(Deserialize)]
pub struct RawDoughSpec {
    pub name: String,
    pub description: Option<String>,
    pub flour: HashMap<String, f32>,
    pub hydration: HashMap<String, f32>,
    pub ingredients: HashMap<String, f32>,
    pub starter: Option<f32>,
}

impl TryFrom<RawDoughSpec> for DoughSpec {
    type Error = &'static str;
    fn try_from(value: RawDoughSpec) -> Result<Self, Self::Error> {
        let bakers_flour = value
            .flour
            .into_iter()
            .map(|(name, amount)| {
                (
                    name,
                    BakersPercent {
                        name: name.replace("_", " "),
                        percent: amount.into(),
                    },
                )
            })
            .collect();

        let bakers_hydration = value
            .hydration
            .into_iter()
            .map(|(name, amount)| {
                (
                    name,
                    BakersPercent {
                        name: name.replace("_", " "),
                        percent: amount.into(),
                    },
                )
            })
            .collect();

        let bakers_ingredients = value
            .ingredients
            .into_iter()
            .map(|(name, amount)| BakersPercent {
                name: name.replace("_", " "),
                percent: amount.into(),
            })
            .collect();

        let bakers_starter = value.starter.map(|amount| BakersPercent {
            name: "Starter".into(),
            percent: amount.into(),
        });

        Ok(DoughSpec {
            name: value.name,
            description: value.description,
            flour: bakers_flour,
            hydration: bakers_hydration,
            ingredients: bakers_ingredients,
            starter: bakers_starter,
        })
    }
}

#[derive(Deserialize, Debug)]
#[serde(try_from = "RawStarterSpec")]
pub struct StarterSpec {
    pub flour: IngredientMap,
    pub hydration: Percent,
}

/// Validation on starter specification
#[derive(Deserialize)]
pub struct RawStarterSpec {
    pub flour: HashMap<String, f32>,
    pub hydration: f32,
}

impl TryFrom<RawStarterSpec> for StarterSpec {
    type Error = &'static str;
    fn try_from(value: RawStarterSpec) -> Result<Self, Self::Error> {
        let bakers_flour = value
            .flour
            .into_iter()
            .map(|(name, amount)| {
                (
                    name,
                    BakersPercent {
                        name: name.replace("_", " "),
                        percent: amount.into(),
                    },
                )
            })
            .collect();

        // Idk why it makes me do this...
        let bakers_hydration = Percent::try_from(value.hydration)?;

        Ok(StarterSpec {
            flour: bakers_flour,
            hydration: bakers_hydration,
        })
    }
}

fn make_percent(mapping: HashMap<String, f32>) -> HashMap<String, f32> {
    HashMap::from_iter(mapping.into_iter().map(|(key, value)| (key, value / 100.0)))
}
