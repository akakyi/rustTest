//use std::io;

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
}
