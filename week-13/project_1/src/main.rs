use std::io;
use std::fs;

fn main() {
    println!("Choose your role (1-5):");
    println!("1. Admin");
    println!("2. Project Manager");
    println!("3. Employee");
    println!("4. Customer");
    println!("5. Vendor");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Error reading input");

    let role: u32 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input. Please enter a number.");
            return;
        }
    };

    let file_name = if role == 1 {
        "globacom_dbase.sql"
    } else if role == 2 {
        "project_table.sql"
    } else if role == 3 {
        "staff_table.sql"
    } else if role == 4 {
        "customer_table.sql"
    } else if role == 5 {
        "dataplan_table.sql"
    } else {
        println!("choose a valid role from the options");
        return;
    };

    let content = fs::read_to_string(file_name);
    match content {
        Ok(data) => println!("{}", data),
        Err(_) => println!("Could not open file: {}", file_name),
    }
}
