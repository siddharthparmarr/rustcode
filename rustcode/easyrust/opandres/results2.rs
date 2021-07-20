fn main() {
    let weather_vec = vec![
        vec!["Berlin", "cloudly", "5", "-7", "78"],
        vec!["Atgens", "sunny", "not humid", "30", "45", "7"],
    ];
    for mut city in weather_vec {
        println!("for the city of {}", city[0]);

        while let Some(information) = city.pop() {
            //this means keep going until you can't pop
            //when the vector reaches 0 item, it will return None
            //and it will stop
            if let Ok(number) = information.parse::<i32>() {
                //try to parse the variable we called information
                //this returns result if it is ok(number) it will print it
                println!("the number is {}", number);
                //we don't write anthing here cuz we do nothing if we get an err.
                //throw them all away
            }
        }
    }
}
