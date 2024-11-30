use std::io;

fn main() {
    loop {
        println!("Welcome to Karyn's Maths AI. Choose the calculation you want me to solve:");
        println!("Enter your choice:");
        println!("1. Area of Trapezium");
        println!("2. Area of Rhombus");
        println!("3. Area of Parallelogram");
        println!("4. Area of Cube");
        println!("5. Volume of Cylinder");
        println!("6. EXIT");

        let choice: u32 = get_input("Enter your choice (1-6): ").parse().unwrap_or(0);

        if choice == 1 {
            calculate_trapezium();
        } else if choice == 2 {
            calculate_rhombus();
        } else if choice == 3 {
            calculate_parallelogram();
        } else if choice == 4 {
            calculate_cube();
        } else if choice == 5 {
            calculate_cylinder();
        } else if choice == 6 {
            println!("Exiting the program.THank you for choosing our software");
            break;
        } else {
            println!("Invalid choice, please try again.");
        }
    }
}

fn calculate_trapezium() {
    let height: f64 = get_input("Enter height: ").parse().expect("Invalid input for height.");
    let base1: f64 = get_input("Enter base1: ").parse().expect("Invalid input for base1.");
    let base2: f64 = get_input("Enter base2: ").parse().expect("Invalid input for base2.");
    let area = (height / 2.0) * (base1 + base2);
    println!("Area of Trapezium: {}", area);
}

fn calculate_rhombus() {
    let diagonal1: f64 = get_input("Enter diagonal1: ").parse().expect("Invalid input for diagonal1.");
    let diagonal2: f64 = get_input("Enter diagonal2: ").parse().expect("Invalid input for diagonal2.");
    let area = 0.5 * diagonal1 * diagonal2;
    println!("Area of Rhombus: {}", area);
}

fn calculate_parallelogram() {
    let base: f64 = get_input("Enter base: ").parse().expect("Invalid input for base.");
    let altitude: f64 = get_input("Enter altitude: ").parse().expect("Invalid input for altitude.");
    let area = base * altitude;
    println!("Area of Parallelogram: {}", area);
}

fn calculate_cube() {
    let side: f64 = get_input("Enter the side length: ").parse().expect("Invalid input for side.");
    let area = 6.0 * side * side;
    println!("Surface Area of Cube: {}", area);
}

fn calculate_cylinder() {
    let radius: f64 = get_input("Enter radius: ").parse().expect("Invalid input for radius.");
    let height: f64 = get_input("Enter height: ").parse().expect("Invalid input for height.");
    let volume = 3.1415 * radius * radius * height;
    println!("Volume of Cylinder: {}", volume);
}

fn get_input(prompt: &str) -> String {
    let mut input = String::new();
    println!("{}", prompt);
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}
