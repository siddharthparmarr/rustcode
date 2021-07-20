struct Rect {}

struct Circle {}
trait Drawable {
    fn draw(&self);
    fn describe(&self) {
        println!(" default describe");
    }
}

impl Drawable for Circle {
    fn draw(&self) {
        println!("Draw circle");
    }

    fn describe(&self) {
        println!(" default circle");
    }
}

impl Drawable for Rect {
    fn draw(&self) {
        println!("Draw rect1");
    }

    fn describe(&self) {
        println!(" default rectangle");
    }
}
impl Drawable for String {
    fn draw(&self) {
        println!("{}", self);
    }
    fn describe(&self) {
        println!("default string");
    }
}
trait AlsoDraw {
    fn draw(&self) {}
}
impl AlsoDraw for Rect {}
fn main() {
    let rect = Rect {};
    AlsoDraw::draw(&rect);
    let circle = Circle {};
    let shapes: Vec<Box<dyn Drawable>> = vec![
        Box::new(rect),
        Box::new(circle),
        Box::new("Hellow wordl".to_string()),
    ];
    for shape in shapes {
        shape.draw();
        shape.describe();
    }
}
