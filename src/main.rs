//use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Write \"Test\"");

    let mut input = String::new();

    let bytesCount = std::io::stdin()
        .read_line(&mut input)
        .expect("This is error msg");

    println!("bytes writed: {bytesCount}");
    if (input == "Test") {
        println!("Da")
    } else {
        println!("Zochem {input}?")
    }

    let secretNumber = rand::thread_rng().gen_range(1..=100);
    let input = input.trim().parse::<i32>().expect("That is not a number!");
//    if (inputInt == secretNumber) {
//        println!("Ugadal")
//    } else {
//        println!("Ne ugadal)))00)");
//    }
    match input.cmp(&secretNumber) {
        Ordering::Less => println!("You entered less"),
        Ordering::Equal => println!("Ugadal)))00)"),
        Ordering::Greater => println!("You entered less")
    }
}
