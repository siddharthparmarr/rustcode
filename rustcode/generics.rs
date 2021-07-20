struct World<T> {
    player: T,
}
struct Player {
    hp: u32,
}
struct DebugPlayer {
    inner: Player,
}
trait Damage {
    fn take_damage(&mut self, damage: u32);
}
impl<T: Damage> World<T> {
    fn damage_player(&mut self) {
        self.player.take_damage(10);
    }
}

impl Damage for Player {
    fn take_damage(&mut self, damage: u32) {
        self.hp -= damage;
    }
}

impl Damage for DebugPlayer {
    fn take_damage(&mut self, damage: u32) {
        println!("Damage: {}", damage);
        self.inner.take_damage(damage);
    }
}
fn use_world<T: Damage>(mut world: World<T>) {
    world.player.take_damage(10);
}
fn main() {
    let mut world = World {
        player: DebugPlayer {
            inner: Player { hp: 100 },
        },
    };
    world.damage_player();
}
