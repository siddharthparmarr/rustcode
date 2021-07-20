fn returns_strS() -> &'static str {

let my_string = String::from("i am a string");
"i am a string"
}
fn main() {

let my_str = returns_strS();
println!("{}", my_str);

}
