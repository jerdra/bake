use crate::spec::FlourMap;
use crate::ingredients::CalculatedIngredient;

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


