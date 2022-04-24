use crate::spec::{DoughSpec, StarterSpec};
/// Compute bread formulas using a recipe
/// and output the required ingredients weights
use std::collections::HashMap;

struct CalculatedIngredient {
    name: String,
    weight: f32,
}

struct Starter {
    amount: f32,
    flour: HashMap<String, f32>,
    hydration: f32,
}

/// Full Dough recipe specification with or without a starter included
pub struct Formula {
    name: String,
    hydration: f32,
    salt: f32,
    flours: HashMap<String, f32>,
    extras: Option<HashMap<String, f32>>,
    description: Option<String>,
    starter: Option<Starter>,
}

impl Formula {
    pub fn new(dough_spec: DoughSpec, maybe_starter_spec: Option<StarterSpec>) -> Formula {
        let starter: Option<Starter> = match (dough_spec.starter, maybe_starter_spec) {
            (Some(amt), Some(spec)) => Some(Starter {
                amount: amt,
                flour: spec.flour,
                hydration: spec.hydration,
            }),
            (Some(_), None) => panic!("ERROR: No starter spec, yet dough requires starter!"),
            _ => None,
        };

        Formula {
            name: dough_spec.name,
            flours: dough_spec.flour,
            hydration: dough_spec.hydration,
            salt: dough_spec.salt,
            description: dough_spec.description,
            extras: dough_spec.extras,
            starter,
        }
    }

    fn calculate_water(&self, total_flour: f32) -> f32 {
        match &self.starter {
            Some(starter) => total_flour * (self.hydration - starter.hydration * starter.amount),
            None => total_flour * self.hydration,
        }
    }

    fn calculate_mixins(&self, total_flour: f32) -> Option<Vec<CalculatedIngredient>> {

        self.extras.as_ref().map(|extras| {
                extras
                    .iter()
                    .map(|(key, value)| CalculatedIngredient {
                        name: key.to_string(),
                        weight: total_flour * value,
                    })
                    .collect()
        })


    }

    /// Convert a bread Formula into a bread Recipe
    pub fn into_recipe(self, weight: f32) -> Recipe {
        let total_flour = weight / (1f32 + self.hydration + self.salt);

        let salt = CalculatedIngredient {
            name: "Salt".to_string(),
            weight: self.salt * total_flour,
        };

        let water = CalculatedIngredient {
            name: "Water".to_string(),
            weight: self.calculate_water(total_flour),
        };
        let mixins = self.calculate_mixins(total_flour);
        let flours = self.calculate_flour(total_flour);

        Recipe {
            name: self.name,
            total_weight: weight,
            flours,
            mixins,
            water,
            salt,
            description: self.description,
        }
    }

    fn calculate_flour(&self, total_flour: f32) -> Vec<CalculatedIngredient> {


        if let Some(starter) = &self.starter {
            unimplemented!()
        } else {
            self.flours
                .iter()
                .map(|(flour, amt)| CalculatedIngredient {
                    name: flour.to_string(),
                    weight: amt * total_flour,
                })
                .collect()
        }
    }
}

pub struct Recipe {
    name: String,
    total_weight: f32,
    flours: Vec<CalculatedIngredient>,
    mixins: Option<Vec<CalculatedIngredient>>,
    water: CalculatedIngredient,
    salt: CalculatedIngredient,
    description: Option<String>,
}

impl Recipe {}
