struct Position(u32, i32);
struct Player {
    name: String,
    hp: u32,
    location: Position,
}
impl Player {
    fn describe(&self) {
        println!("Name: {} | Hp: {}", self.name, self.hp);
        println!("Pos x: {} y: {}", self.location.0, self.location.1);
    }
    fn new(name: String) -> Self {
        Self {
            name,
            hp: 100,
            location: Position(1, 1),
        }
    }
}
fn die(
    Player {
        location: Position(x, y),
        name,
        ..
    }: Player,
) {
    println!("{} died at {} {} ", name, x, y);
}
fn main() {
    let player = Player::new("bob".to_string());
    player.describe();
    die(player);
}
