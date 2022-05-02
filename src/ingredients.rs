use std::fmt::Display;

#[derive(Debug)]
pub struct CalculatedIngredient {
    pub name: String,
    pub weight: f32,
}

impl Display for CalculatedIngredient {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {:.2}", self.name, self.weight)
    }
}

impl PartialEq for CalculatedIngredient {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.weight == other.weight
    }
}
