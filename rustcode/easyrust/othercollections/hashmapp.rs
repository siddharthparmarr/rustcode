use std::collections::BTreeMap;
//hash map is not in order
//if you want things in order replace hashmap with BTreeMap

struct City {
    name: String,
    population: BTreeMap<u32, u32>,
}
fn main() {
    let mut tallinn = City {
        name: "Tallinn".to_string(),
        population: BTreeMap::new(), // so far the hash map is empty
    };

    tallinn.population.insert(137, 3_250);
    tallinn.population.insert(1343, 334_250);
    tallinn.population.insert(2021, 333_250);
    for (year, population) in tallinn.population {
        println!(
            "in the year {} the city of {} had a population of {}",
            year, tallinn.name, population
        );
    }
}
