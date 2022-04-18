/// Compute bread formulas using a recipe
/// and output the required ingredients weights

pub enum BakersMeasurement {

    // Flour percent should sum to 100
    Flour(f32),

    // Bakers percent relative to flour weight
    Bakers(f32),

    // Total percent relative to total dough weight
    Total(f32),

    // Starter amount
    Starter(f32),
}

pub struct Ingredient {
    name: String,
    measurement: BakersMeasurement
}

struct CalculatedIngredient {
    name: String,
    weight: f32
}

impl Ingredient {
    pub fn new(name: String, measurement: BakersMeasurement) -> Ingredient {
        Ingredient { name: name, measurement }
    }
}

pub struct Formula {
    ingredients: Vec<Ingredient>
}

impl Formula {
    pub fn new(ingredients: Vec<Ingredient>) -> Formula {
        Formula { ingredients: Vec::new() }
    }

    pub fn compute(&self, weight: u32) -> Recipe {
        // Do stuff to make a recipe
        unimplemented!()
    }
}

struct Recipe {
    name: String,
    total_weight: u32,
    ingredients: Vec<CalculatedIngredient>
}
