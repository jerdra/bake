Bake
======

------------
**WARNING**: This repository is still very much in progress (missing features)

- [ ] Allow yeast key to be used
- [ ] Use a slightly different adjustment algorithm? (least squares solution vs. naive re-allocation?) Alteratively provide an option
------------

Simple little CLI tool for converting Bread bakers formulas into full recipes using Rust!

## Basic Usage

`bake` requires the user to specify their dough formula using a `YAML` file. The YAML file is a simple text file that `bake` reads and convert into a bread recipe. Below is an example YAML file:

`basic.yaml`
```
name: Basic Dough
description: This is a very basic 80% hydration, 5% whole wheat recipe

flour:
  white: 95
  whole_wheat: 5

hydration: 80
salt: 2
```

With this `formula`, `bake` can convert this into a 1000g bread recipe using the following command:

```
bake --formula basic.yaml --weight 1000
```

This outputs:

```
Dough Recipe
============================
This is a very basic 80% hydration, 5% whole wheat recipe
============================

Basic Dough
Total Weight: 1000.00

Flours:
----------------------------
whole_wheat: 27.47
white: 521.98
----------------------------

Water: 439.56
Salt: 10.99


Final Dough Composition:
===========================
Flours:
---------------------------
whole_wheat: 0.05
white: 0.95
---------------------------

Hydration: 80.00
Salt: 2.00
```

### Sourdough Recipes

Sourdough recipes become complicated as they introduce additional flours into the final dough that is produced. `bake` will attempt to re-adjust flours in order to preserve the baker's intended dough formula while accounting for any new flours introduced by the starter. This in accordance with the [BBGA Bakers Formula Guide](https://www.bbga.org/files/2009FormulaFormattingSINGLES.pdf). To introduce a Sourdough culture into your recipe, you'll need another YAML file describing your starter:

`starter.yaml`
```
flour:
  white: 50
  whole_wheat: 50

hydration: 100
```

**NOTE**: The `flour`s in your starter are matched up to the original recipe by their names! Make sure you don't have any typos

Let's update our `basic.yaml` to reflect how much starter we want to add:

`basic-with-starter.yaml`
```
name: Basic Dough
description: Basic 80% sourdough recipe

flour:
  white: 95
  whole_wheat: 5

# Add starter percent
starter: 15 
hydration: 80
salt: 2
```

With our starter described in `starter.yaml` and incorporated into `basic-with-starter.yaml`, let's `bake`:
```
bake --formula basic-with-starter.yaml --starter-spec starter.yaml --weight 1000
```

Output:
```
Dough Recipe
============================
Basic 80% sourdough recipe
============================

Basic Dough
Total Weight: 1000.00

Flours:
----------------------------
whole_wheat: 6.87
white: 501.37
----------------------------

Water: 398.35
Salt: 10.99

Starter:
----------------------------
Total Amount: 82.42

        white: 20.60
        whole_wheat: 20.60

        Water: 41.21
----------------------------


Final Dough Composition:
===========================
Flours:
---------------------------
whole_wheat: 0.01
white: 0.91
Prefermented white: 3.75
Prefermented whole_wheat: 3.75
---------------------------

Hydration: 80.00
Salt: 2.00
```

### What about Mixins?

Mixins can be added to any bread formula simply by specifying the `mixins` key in your formula:


```
name: Onion garlic bread
flour:
  white: 100

mixins:
  onion: 5
  garlic: 2

hydration: 80
salt: 2

description: |
  "Onion garlic bread"

```

Output:
```
Dough Recipe
============================
"Onion garlic bread"

============================

Onion garlic bread
Total Weight: 1000.00

Flours:
----------------------------
white: 529.10
----------------------------

Water: 423.28
Salt: 10.58

Mix-ins:
----------------------------
onion: 26.46
garlic: 10.58
----------------------------


Final Dough Composition:
===========================
Flours:
---------------------------
white: 1.00
---------------------------

Hydration: 80.00
Salt: 2.00

Mixins:
---------------------------
onion: 5.00
garlic: 2.00
---------------------------
```
