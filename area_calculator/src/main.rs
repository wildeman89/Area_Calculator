fn main() {
    let mut run: bool = true;
    while run {
        intro();
        let option: f64 = input("Make your selection: ");
        // going to collect the value of run from match
        run = match option as usize {
            1 => {
                rectangle();
                true
            },
            2 => {
                triangle();
                true
            },
            3 => {
                circle();
                true
            },
            4 => {
                println!("Goodbye!\n");
                false
            },
            _ => {
                println!("Invalid option. Choose from the menu options.");
                true
            },
        };
    }
    
}

// function declarations
fn intro() {
    println!("\n\t\tWelcome to the Area Calculator");
    println!("\nThis program will calculate the area of a shape that you choose.");
    println!("Please choose one of the following shapes:");
    println!("\t1)Rectangle/Square\n\t2)Triangle\n\t3)Circle\n\t4)QUIT");
}

fn convert_to_float(value: &String) -> f64 {
    let value: f64 = value.trim().parse().expect("Failed to parse to f64");
    value
}

fn input(prompt: &str) -> f64 {
    use std::io::{Write, stdin, stdout};
    print!("{}", prompt);
    stdout().flush().unwrap();
    let mut choice = String::new();
    stdin().read_line(&mut choice).expect("Failed to get user input");
    let choice: f64 = convert_to_float(&choice);
    choice
}

fn rectangle() {
    println!("\n\t\tRectangle/Square Area Calculation");
    println!("Enter the following dimensions in order to get the area of your shape.");
    let length = input("Enter the length of the Rectangle/Square: ");
    let width = input("Enter the width of the Rectangle/Square: ");
    let area = length * width;
    println!("Length: {:.2}\nWidth: {:.2}\nArea: {:.2}", length, width, area);
}

fn triangle() {
    println!("\n\t\tTriangle Area Calculator");
    println!("Enter the following dimensions in order to get the area of the triangle.");
    let base = input("Enter the base of the triangle: ");
    let height = input("Enter the height of the triangle: ");
    let area = (base * height) / 2.0;
    println!("Base: {:.2}\nHeight: {:.2}\nArea: {:.2}", base, height, area);
}

fn circle() {
    println!("\n\tCircle Area Calculator");
    println!("Enter the following dimension to get the area of your circle.");
    let radius = input("Enter the Radius of the circle: ");
    let pi: f64 = 22.0 / 7.0;
    let area = pi * f64::powf(radius, 2.0);
    println!("Radius: {:.4}\nPI: {:.4}\nArea: {:.4}", radius, pi, area);
}