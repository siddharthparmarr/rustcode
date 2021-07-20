use std::collections::HashMap;

fn main() {
    let canadian_cities = vec!["calgary", "vancouver", "gimli"];
    let german_cities = vec!["karlsruhe", "bad doberan", "bielefeld"];

    let mut city_hashmap = HashMap::new();

    for city in canadian_cities {
        city_hashmap.insert(city, "canada");
    }
    for city in german_cities {
        city_hashmap.insert(city, "Germany");
    }

    println!("{:?}", city_hashmap["bielefeld"]);
    println!("{:?}", city_hashmap.get("bielefeld"));
    println!("{:?}", city_hashmap.get("bielefelddd"));
}
