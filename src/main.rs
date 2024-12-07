use std::io;

fn main() {
    kisnagy();
}

fn kisnagy() {
    println!("First number: ");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let number1: i32 = input.trim().parse().expect("Please type in a number");

    println!("Second number: ");
    input.clear();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let number2: i32 = input.trim().parse().expect("Please type in a number");

    let (bigger, smaller) = if number1 > number2 {
        (number1, number2)
    } else {
        (number2, number1)
    };

    let remain = bigger % smaller;
    let outcome = bigger / smaller;

    println!(
        "The larger number contains the smaller number {} times, with a remainder of {}.",
        outcome, remain
    );
}
