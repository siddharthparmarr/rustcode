use std::fmt;

struct Cat {
    name: String,
    age: u8,
}
impl fmt::Display for Cat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} is a cat who is {} years old.", self.name, self.age)
    }
}
fn print_cats(pet: String) {
    println!("{}", pet);
}
fn main() {
    let mr_mantle = Cat {
        name: "sid".to_string(),
        age: 4,
    };

    print_cats(mr_mantle.to_string());
    println!(
        "mr. mantle's string is {} letters long",
        mr_mantle.to_string().chars().count()
    );
}
