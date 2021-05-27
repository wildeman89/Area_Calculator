// Going to place structs at the top
struct Rectangle {
    length: f64,
    width: f64,
}

impl Rectangle {
    fn area(&self) -> f64 {
        self.length * self.width
    }
}

struct Triangle {
    base: f64,
    height: f64,
}

impl Triangle {
    fn area(&self) -> f64 {
        (self.base * self.height) / 2.0
    }
}

struct Circle {
    radius: f64,
}

impl Circle {
    fn area(&self) -> f64 {
        let pi: f64 = 22.0 / 7.0;
        pi * f64::powf(self.radius, 2.0)
    }
}


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
    println!("\t1) Rectangle/Square\n\t2) Triangle\n\t3) Circle\n\t4) QUIT");
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
    let rect = create_rectangle(length, width);
    println!("Length: {:.2}\nWidth: {:.2}\nArea: {:.2}", rect.length, rect.width, rect.area());
}

fn triangle() {
    println!("\n\t\tTriangle Area Calculator");
    println!("Enter the following dimensions in order to get the area of the triangle.");
    let base = input("Enter the base of the triangle: ");
    let height = input("Enter the height of the triangle: ");
    let tri = create_triangle(base, height);

    println!("Base: {:.2}\nHeight: {:.2}\nArea: {:.2}", tri.base, tri.height, tri.area());
}

fn circle() {
    println!("\n\tCircle Area Calculator");
    println!("Enter the following dimension to get the area of your circle.");
    let radius = input("Enter the Radius of the circle: ");
    let circ = create_circle(radius);
    println!("Radius: {:.4}\nArea: {:.4}", circ.radius, circ.area());
}

fn create_rectangle(length: f64, width: f64) -> Rectangle {
    let rect = Rectangle {
        length,
        width,
    };
    rect
}

fn create_triangle(base: f64, height: f64) -> Triangle {
    let tri = Triangle {
        base,
        height,
    };
    tri
}

fn create_circle(radius: f64) -> Circle {
    let circ = Circle {
        radius,
    };
    circ
}