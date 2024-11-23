// CA QUESTION 1

use std::io;
fn main() {
	let mut input1 = String::new();
	let mut input2 = String::new();
	
	let mut input3 = String::new();


	let mut name = String::new();
	let mut email = String::new();
	let mut department = String::new();
	let mut state_of_origin = String::new();

	println!("Enter student level");
	io::stdin().read_line(&mut input1).expect("failed to read_line");
    let level:u32 = input1.trim().parse().expect("enter a valid number");

    println!("Enter your cgpa");
   io::stdin().read_line(&mut input2).expect("failed to read_line");
    let cgpa:f32 = input2.trim().parse().expect("enter a valid number");
    	  
    	  println!("Are you a class rep");
    	 io::stdin().read_line(&mut input3).expect("failed to read_line");
    	 let class_rep:bool  = input3.trim().parse().expect("failed to input");


    println!("Enter name");
	io::stdin().read_line(&mut name).expect("failed to read_line");

	println!("Enter your email");
	io::stdin().read_line(&mut email).expect("not a valid string");

	println!("Enter your department");
	io::stdin().read_line(&mut department).expect("not a valid string");

	println!("Enter your state_of_origin ");
	io::stdin().read_line(&mut state_of_origin).expect("not a valid string");

	if class_rep && level == 100 && cgpa > 4.0 {
		println!("You can vote");
		println!("student details:",);
		println!("name: {}", name.trim());
		println!("Department: {}",department.trim());
		println!("state_of_origin: {}",state_of_origin.trim());

	}else {
		println!("sorry, you are not eligible to vote");
	}
}

// CA QUESTION 2
use std::io;
fn main() {

	let mut input1 = String::new();
    let mut input2 = String::new();

    println!("Enter your name");
    io::stdin().read_line(&mut input1).expect("Not a valid string");

    println!("Enter number of paper published");
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let no_published:i32= input2.trim().parse().expect("Not a valid number");

    if no_published>=3 && no_published <=5{
    	println!("My incentive is N500,000 {}",input1);
    }
	else if no_published>=5 && no_published <=10{
		println!("My incentive is N800,000 {}",input1);
	}
	else if no_published>=10{
		println!("My incentive is N1,000,000 {}",input1);
}
    else if no_published <3 {
	            println!("My incentive is N100,000 {}",input1);
}
}  



