use std::io;
use std::cmp::Ordering;

fn main() {

    println!("Which Fibonacci number do you want ?");

    let mut ask_fibonacci_number = String::new();

    io::stdin()
        .read_line(&mut ask_fibonacci_number)
        .expect("Failed to read line");

    let (occurrence, mut  actual_fibonacci_number, mut last_fibonacci_number, mut fibonacci_number): (u32, i32, i32, i32) = (1, 1, 0, 1);

    let temporary_occurence = occurrence;
    let mut new_occurence: u32 = temporary_occurence.to_string().trim().parse().unwrap();
    let ask_fibonacci_number: u32 =  ask_fibonacci_number.trim().parse().unwrap();

    loop {

        match new_occurence.cmp(&ask_fibonacci_number) {
            Ordering::Less => {
                fibonacci_number = actual_fibonacci_number + last_fibonacci_number;
                last_fibonacci_number = actual_fibonacci_number;
                actual_fibonacci_number = fibonacci_number;
                new_occurence += 1;
            },
            Ordering::Greater => {
                println!("Too big! {}", occurrence);
                break;
            },
            Ordering::Equal => {
                println!("Finish");
                println!("The Fibonacci number for the occurrence {}, is {}", new_occurence, fibonacci_number);
                break;
            }
        }
    }
}