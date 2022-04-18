use crate::recipe::{BakersMeasurement, Formula, Ingredient};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct FormulaSpec {
    flour: HashMap<String, f32>,
    bakers: HashMap<String, f32>,
    starter: Option<f32>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StarterSpec {
    flour: HashMap<String, f32>,
    bakers: HashMap<String, f32>,
}

impl FormulaSpec {

    /// Consumes FormulaSpec and transforms into a Formula
    pub fn into_formula(self) -> Formula {

        let mut ingredients = Vec::new();

        for (flour, percent) in self.flour {
            ingredients.push(Ingredient::new( flour, BakersMeasurement::Flour(percent)));
        }

        for (ingredient, percent) in self.bakers{
            ingredients.push(Ingredient::new( ingredient, BakersMeasurement::Bakers(percent)));
        }

        if let Some(percent) = self.starter {
            ingredients.push(Ingredient::new( "starter".into(), BakersMeasurement::Starter(percent)));
        }

        Formula::new ( ingredients )
    }
}
