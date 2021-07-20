use std::cmp::PartialOrd;
use std::fmt::Display;
// we want to compare them so we need partialord. that trait let us use things like <, >, ==, etc.
// we want to print them too so we require Display for U as well.

fn compare_and_display<T: Display, U: Display + PartialOrd>(statement: T, num_1: U, num_2: U) {
    println!(
        "{} is {} greater than {}? {}",
        statement,
        num_1,
        num_2,
        num_1 > num_2
    );
}
fn main() {
    compare_and_display("Listen up!", 8, 6);
}
