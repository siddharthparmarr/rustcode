struct Person {
    name: String,
    read_name: String,
    height: u8,
    happiness: bool,
}
fn main() {
    let papa_doc = Person {
        name: "papa_doc".to_string(),
        read_name: "clarence".to_string(),
        height: 170,
        happiness: false,
    };
    let Person {
        name: a,
        read_name: b,
        height: c,
        happiness: d,
    } = papa_doc;
    println!(
        "they call him {} but his real name is {}. He is {} cm tall and is he happy? {}",
        a, b, c, d
    );
}
