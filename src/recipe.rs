use crate::spec::{DoughSpec, FlourMap, StarterSpec};
/// Compute bread formulas using a recipe
/// and output the required ingredients weights
use std::collections::HashMap;

// For storing flour maps
#[derive(Debug)]
struct CalculatedIngredient {
    name: String,
    weight: f32,
}

struct Starter {
    amount: f32,
    flour: FlourMap,
    hydration: f32,
}

/// Full Dough recipe specification with or without a starter included
pub struct Formula {
    name: String,
    hydration: f32,
    salt: f32,
    flours: FlourMap,
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
            Some(starter) => {
                total_flour * (self.hydration - (1.0 / (1.0 + starter.hydration)) * starter.amount)
            }
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
        // Need mixins here
        let total_flour = weight
            / (1f32
                + self.hydration
                + self.salt
                + self.extras.as_ref().map_or(0.0, |extras| {
                    let mut result = 0.0;
                    for value in extras.values() {
                        result += value
                    }
                    result
                }));

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
        // TODO: Simplify the shit outta this
        if let Some(starter) = &self.starter {
            let starter_amt = starter.amount * total_flour;
            let flour_amt = total_flour - starter_amt;

            let adjusted_flour =
                adjust_for_starter(flour_amt, &self.flours, starter_amt, &starter.flour);
            adjusted_flour
                .iter()
                .map(|(flour, amt)| CalculatedIngredient {
                    name: flour.to_string(),
                    weight: *amt,
                })
                .collect()
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

#[derive(Debug)]
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

/// Dough adjustment algorithm
/// 1. Subtract out flours that only appear in Starter
/// 2. With remaining amount, compute new proportions adjusted for starter contribution
/// 3. If no negative amounts appear, we're done
/// 4. If negative amounts appear, subtract amount from remaining amount
/// 5. Set negative values to 0, and remove from consideration
/// 6. Go to step 2.
fn adjust_for_starter(
    dough_amt: f32,
    dough_flour: &FlourMap,
    starter_amt: f32,
    starter_flour: &FlourMap,
) -> FlourMap {
    let mut amt_to_redistribute: f32 = dough_amt + starter_amt;
    let mut final_dough_amts = HashMap::new();

    // Initialization should only be for starter keys
    for key in starter_flour.keys() {
        if !dough_flour.contains_key(key) {
            let starter_amt = starter_flour.get(key).unwrap_or(&0.0) * starter_amt;
            amt_to_redistribute -= starter_amt;
            final_dough_amts.insert(key.to_string(), starter_amt);
        }
    }

    let mut vars_to_adjust: Vec<&String> = dough_flour.keys().collect();

    let mut found_negative;
    loop {
        found_negative = false;
        for key in &vars_to_adjust {
            // adjustment = (raw proportion in remaining) - (starter contribution of flour type)
            // adj_X = (d_x * T') - (s_x * S)
            let adjustment_amt = dough_flour.get(*key).unwrap_or(&0.0) * amt_to_redistribute
                - starter_flour.get(*key).unwrap_or(&0.0) * starter_amt;

            final_dough_amts.insert(key.to_string(), adjustment_amt);
            found_negative = adjustment_amt < 0.0;
        }

        if !found_negative {
            break;
        }

        final_dough_amts.iter_mut().for_each(|(key, value)| {
            if *value < 0.0 {
                amt_to_redistribute -= *value;
                vars_to_adjust.retain(|x| *x != key);
                *value = 0.0;
            }
        });
    }

    // This gives final composition
    // Not the amount that we need to add.... should be calculated separately
    FlourMap(final_dough_amts)
}
