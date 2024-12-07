use std::io;

fn main() {
    //kisnagy();
    haromszog1();
}

fn haromszog1() {
    println!("First number:");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let a: i32 = input.trim().parse().expect("Please type in a number!");

    println!("Second number:");
    input.clear();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let b: i32 = input.trim().parse().expect("Please type in a number!");

    println!("Third number:");
    input.clear();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let c: i32 = input.trim().parse().expect("Please type in a number!");

    let (largest, middle_sized, smallest) = if a >= b && a >= c {
        (a, b, c)
    } else if b >= a && b >= c {
        (b, c, a)
    } else {
        (c, a, b)
    };

    if middle_sized + smallest > largest {
        println!("Triangle.");
        if (middle_sized*middle_sized + smallest*smallest) == largest*largest {
            println!("Right-angled.");
        } else {
            println!("Not right-angled.");
        }
    } else {
        println!("Not a triangle.");
    }
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
