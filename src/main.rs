use std::fs::File;

mod recipe;
mod spec;

use clap::Parser;
use crate::spec::{FormulaSpec, StarterSpec};

/// Create Bread recipes using JSON formulas
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {

    /// YAML file containing dough formula
    #[clap(short, long)]
    formula: String,

    /// YAML file containing starter build
    #[clap(short, long)]
    starter_spec: String,

    /// Target dough weight
    #[clap(short, long)]
    weight: f32,

    /// Save final recipe to a file
    #[clap(short='o', long)]
    save_to: Option<String>
}



fn main() {

    let args = Args::parse();

    let formula_file = File::open(args.formula).unwrap();
    let formula: FormulaSpec = serde_yaml::from_reader(formula_file).unwrap();

    let starter_file = File::open(args.starter_spec).unwrap();
    let starter_spec: StarterSpec = serde_yaml::from_reader(starter_file).unwrap();

    println!("{:#?}", formula);
    println!("{:#?}", starter_spec);

    // STEP 1: Need to build a Formula
    let formula = formula.into_formula();

    // API

    // Parse JSON file

    // Create Ingredients

    // Add to Recipe

    // Compute
}
