fn match_colours(rbg: (i32, i32, i32)) {
    println!(
        "Comparing a colour with {} red {} blue, and {} green",
        rbg.0, rbg.1, rbg.2
    );
    let new_vec = vec![(rbg.0, "red"), (rbg.1, "blue"), (rbg.2, "green")];
    let mut all_have_at_least_10 = true;
    for item in new_vec {
        if item.0 < 10 {
            all_have_at_least_10 = false;
            println!("Not much {}", item.1)
        }
    }
    if all_have_at_least_10 {
        println!("each colour has atleast 10");
    }
    println!();
}
fn main() {
    let first = (200, 0, 0);
    let second = (50, 50, 50);
    let third = (500, 50, 0);

    match_colours(first);

    match_colours(second);
    match_colours(third);
}
