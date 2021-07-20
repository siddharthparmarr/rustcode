fn take_fifth(value: Vec<i32>) -> Option<i32> {
    if value.len() < 5 {
        None
    } else {
        Some(value[4])
    }
}
/*fn handle_option(my_option: Vec<Option<i32>>) {
    for item in my_option {
        match item {
            Some(number) => println!("found {}", number),
            None => println!("found a none"),
        }
    }
} instead of this we can use is_some() method*/
fn main() {
    let new_vec = vec![2, 3];
    let bigger_vec = vec![1, 2, 3, 4, 5];
    /*
    let mut option_vec = Vec::new();

    option_vec.push(take_fifth(new_vec));
    option_vec.push(take_fifth(bigger_vec));

    handle_option(option_vec);*/

    let vec_of_vecs = vec![new_vec, bigger_vec];
    for vec in vec_of_vecs {
        let inside_number = take_fifth(vec);
        if inside_number.is_some() {
            println!("we got: {}", inside_number.unwrap());
        } else {
            println!("we got nothing");
        }
    }
}
