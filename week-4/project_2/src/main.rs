use std::io;
fn main() {
   let mut input = String::new();
   println!("Is the employee experience? (yes/no): "); 
   io::stdin().read_line(&mut input).expect("Failed to read line");
   let experienced = input.trim().eq_ignore_ascii_case("yes");
    let incentive:i32;
    if experienced {
        input.clear();
        println!("Enter the age of the employee: ");
        io::stdin().read_line(&mut input).expect("failed to read line");
        let age:i32 = input.trim().parse().expect("Please enter a valid number");
        //determin the incentive based on age
        if age >= 40 {
            incentive = 1_560_000;

        }else if age >= 30 && age < 40 {
            incentive = 1_480_000;

        }else if age < 28 {
            incentive = 1_300_00 * 12; //monthly to annual

        }else {
            incentive = 0;
        }

    }else {
        incentive = 100_000;

    }
    println!("The annual incentive for the employee is: N{}",incentive );
}
