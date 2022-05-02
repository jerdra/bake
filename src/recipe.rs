use crate::spec::{DoughSpec, FlourMap, StarterSpec};
use crate::ingredients::CalculatedIngredient;
use crate::starter::{Starter, CalculatedStarter};
use std::collections::HashMap;
use std::fmt::Display;


/// Full Dough recipe specification with or without a starter included
#[derive(Debug)]
pub struct Formula {
    name: String,
    hydration: f32,
    salt: f32,
    yeast: Option<f32>,
    flours: FlourMap,
    mixins: Option<HashMap<String, f32>>,
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
            mixins: dough_spec.mixins,
            starter,
            yeast: dough_spec.yeast,
        }
    }

    fn calculate_water(&self, total_flour: f32) -> f32 {
        match &self.starter {
            Some(starter) => {
                total_flour * (self.hydration - starter.percent_water() * starter.amount)
            }
            None => total_flour * self.hydration,
        }
    }

    fn calculate_mixins(&self, total_flour: f32) -> Option<Vec<CalculatedIngredient>> {
        self.mixins.as_ref().map(|mixins| {
            mixins
                .iter()
                .map(|(key, value)| CalculatedIngredient {
                    name: key.to_string(),
                    weight: total_flour * value,
                })
                .collect()
        })
    }

    fn calculate_flour(&self, total_flour: f32) -> Vec<CalculatedIngredient> {
        if let Some(starter) = &self.starter {
            let starter_amt = starter.amount * starter.percent_flour() * total_flour;
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

    /// Convert a bread Formula into a bread Recipe
    pub fn into_recipe(self, weight: f32) -> Recipe {
        // Need mixins here
        let total_flour = weight
            / (1f32
                + self.hydration
                + self.salt
                + self.mixins.as_ref().map_or(0.0, |mixins| {
                    let mut result = 0.0;
                    for value in mixins.values() {
                        result += value
                    }
                    result
                })
                + self.yeast.map_or(0.0, |yeast| yeast));

        let salt = CalculatedIngredient {
            name: "Salt".to_string(),
            weight: self.salt * total_flour,
        };

        let water = CalculatedIngredient {
            name: "Water".to_string(),
            weight: self.calculate_water(total_flour),
        };

        let yeast = self.yeast.map(|yeast| CalculatedIngredient {
            name: "Yeast".to_string(),
            weight: yeast * total_flour,
        });

        let mixins = self.calculate_mixins(total_flour);

        let flours = self.calculate_flour(total_flour);

        let starter = self
            .starter
            .map(|starter| starter.into_calculated(total_flour));

        Recipe {
            name: self.name,
            total_weight: weight,
            flours,
            mixins,
            yeast,
            water,
            salt,
            starter,
            description: self.description,
        }
    }
}

/// Provides a composition view on a Recipe
pub struct DoughComposition<'a> {
    total_flour: f32,
    flours: &'a Vec<CalculatedIngredient>,
    water: &'a CalculatedIngredient,
    yeast: &'a Option<CalculatedIngredient>,
    salt: &'a CalculatedIngredient,
    starter: &'a Option<CalculatedStarter>,
    mixins: &'a Option<Vec<CalculatedIngredient>>,
}

impl Display for DoughComposition<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Final Dough Composition:")?;
        writeln!(f, "===========================")?;

        // Now for each flour, check if in starter
        writeln!(f, "Flours: ")?;
        writeln!(f, "---------------------------")?;
        for flour in self.flours {
            writeln!(f, "{}: {:.2}", flour.name, flour.weight / self.total_flour)?;
        }

        if let Some(starter) = self.starter {
            for flour in &starter.flours {
                writeln!(
                    f,
                    "Prefermented {}: {:.2}",
                    flour.name,
                    flour.weight / self.total_flour * 100.0
                )?;
            }
        }

        writeln!(f, "---------------------------")?;
        writeln!(f)?;

        // Water, Salt
        let water_weight = self.water.weight
            + self
                .starter
                .as_ref()
                .map_or(0.0, |starter| starter.water.weight);
        writeln!(
            f,
            "Hydration: {:.2}",
            water_weight / self.total_flour * 100.0
        )?;
        writeln!(
            f,
            "{}: {:.2}",
            self.salt.name,
            self.salt.weight / self.total_flour * 100.0
        )?;

        self.yeast
            .as_ref()
            .map_or(std::fmt::Result::Ok(()), |yeast| {
                writeln!(
                    f,
                    "{}: {:.2}",
                    yeast.name,
                    yeast.weight / self.total_flour * 100.0
                )
            })?;

        writeln!(f)?;

        // Mix-ins
        self.mixins
            .as_ref()
            .map_or(std::fmt::Result::Ok(()), |mixins| {
                writeln!(f, "Mixins:")?;
                writeln!(f, "---------------------------")?;
                for mixin in mixins {
                    writeln!(
                        f,
                        "{}: {:.2}",
                        mixin.name,
                        mixin.weight / self.total_flour * 100.0
                    )?;
                }
                writeln!(f, "---------------------------")?;
                Ok(())
            })?;

        Ok(())
    }
}

#[derive(Debug)]
pub struct Recipe {
    name: String,
    total_weight: f32,
    flours: Vec<CalculatedIngredient>,
    mixins: Option<Vec<CalculatedIngredient>>,
    yeast: Option<CalculatedIngredient>,
    water: CalculatedIngredient,
    salt: CalculatedIngredient,
    starter: Option<CalculatedStarter>,
    description: Option<String>,
}

impl Recipe {
    pub fn view_composition(&self) -> DoughComposition {
        let total_flour: f32 = self.flours.iter().map(|flour| flour.weight).sum::<f32>()
            + self
                .starter
                .as_ref()
                .map_or(0.0, |starter| starter.amount - starter.water.weight);

        DoughComposition {
            total_flour,
            flours: &self.flours,
            mixins: &self.mixins,
            water: &self.water,
            yeast: &self.yeast,
            salt: &self.salt,
            starter: &self.starter,
        }
    }
}

impl Display for Recipe {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self.name)?;
        writeln!(f, "============================")?;
        self.description
            .as_ref()
            .map_or(std::fmt::Result::Ok(()), |description| {
                writeln!(f, "{}", description)
            })?;
        writeln!(f, "============================")?;
        writeln!(f)?;
        writeln!(f, "Total Weight: {:.2}", self.total_weight)?;
        writeln!(f)?;

        writeln!(f, "Flours:")?;
        writeln!(f, "----------------------------")?;
        for ingredient in &self.flours {
            writeln!(f, "{}", ingredient)?;
        }
        writeln!(f, "----------------------------")?;
        writeln!(f)?;

        writeln!(f, "{}", self.water)?;
        writeln!(f, "{}", self.salt)?;
        self.yeast
            .as_ref()
            .map_or(std::fmt::Result::Ok(()), |yeast| writeln!(f, "{}", yeast))?;
        writeln!(f)?;

        self.mixins
            .as_ref()
            .map_or(std::fmt::Result::Ok(()), |mixins| {
                writeln!(f, "Mix-ins:")?;
                writeln!(f, "----------------------------")?;
                for ingredient in mixins {
                    writeln!(f, "{}", ingredient)?;
                }
                writeln!(f, "----------------------------")?;
                writeln!(f)?;
                Ok(())
            })?;

        self.starter
            .as_ref()
            .map_or(std::fmt::Result::Ok(()), |starter| {
                writeln!(f, "Starter:")?;
                writeln!(f, "----------------------------")?;
                writeln!(f, "Total Amount: {:.2}", starter.amount)?;
                writeln!(f)?;
                for ingredient in &starter.flours {
                    writeln!(f, "\t{}", ingredient)?;
                }
                writeln!(f)?;
                writeln!(f, "\t{}", starter.water)?;
                writeln!(f, "----------------------------")?;
                writeln!(f)?;
                Ok(())
            })?;

        Ok(())
    }
}

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
    // Amount of flour to add!
    FlourMap(final_dough_amts)
}
