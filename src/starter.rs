use crate::ingredients::CalculatedIngredient;
use crate::spec::FlourMap;

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn percent_flour_is_less_than_1() {
        let mut flour_map = HashMap::new();
        flour_map.insert("A".to_string(), 0.5);
        flour_map.insert("B".to_string(), 0.5);
        let flour_map = FlourMap(flour_map);
        let starter = Starter {
            amount: 10.0,
            flour: flour_map,
            hydration: 1.0,
        };
        assert!(starter.percent_flour() <= 1.0);
    }

    #[test]
    fn into_calculated_yields_correct_amounts() {
        let mut flour_map = HashMap::new();
        flour_map.insert("A".to_string(), 0.5);
        flour_map.insert("B".to_string(), 0.5);
        let flour_map = FlourMap(flour_map);
        let starter = Starter {
            amount: 0.10,
            flour: flour_map,
            hydration: 1.0,
        };
        let calculated = starter.into_calculated(100.0);

        let expected_flour = vec![
            CalculatedIngredient {
                name: "A".to_string(),
                weight: 2.5,
            },
            CalculatedIngredient {
                name: "B".to_string(),
                weight: 2.5,
            },
        ];
        let difference = calculated
            .flours
            .into_iter()
            .find(|flour| !expected_flour.contains(flour));

        let expected_water = CalculatedIngredient {
            name: "Water".to_string(),
            weight: 5.0,
        };

        assert!(difference.is_none());
        assert_eq!(calculated.amount, 10.0);
        assert_eq!(calculated.water, expected_water);
    }
}

#[derive(Debug)]
pub struct Starter {
    pub amount: f32,
    pub flour: FlourMap,
    pub hydration: f32,
}

impl Starter {
    pub fn percent_flour(&self) -> f32 {
        1.0 - self.percent_water()
    }

    pub fn percent_water(&self) -> f32 {
        1.0 / (1.0 + self.hydration)
    }

    pub fn into_calculated(self, total_flour: f32) -> CalculatedStarter {
        let starter_amount = self.amount * total_flour;
        let flours: Vec<CalculatedIngredient> = self
            .flour
            .iter()
            .map(|(flour, amount)| CalculatedIngredient {
                name: flour.to_string(),
                weight: amount * self.percent_flour() * starter_amount,
            })
            .collect();
        let water = CalculatedIngredient {
            name: "Water".to_string(),
            weight: starter_amount * self.percent_water(),
        };

        CalculatedStarter {
            amount: starter_amount,
            water,
            flours,
        }
    }
}

/// Total amount of starter and flours for composition calculation
#[derive(Debug)]
pub struct CalculatedStarter {
    pub amount: f32,
    pub water: CalculatedIngredient,
    pub flours: Vec<CalculatedIngredient>,
}
