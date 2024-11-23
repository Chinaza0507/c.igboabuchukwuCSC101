use stdin::io;
fn main() {
	let mut input1 = String::new();
	let mut input2 = String::new();
	
	let mut input3 = String::new();


	let mut name = String::new();
	let mut email = string::new();
	let mut department = String::new;
	let mut state of origin = String::new;

	println!("Enter student level");
	io::stdin().read_line(&mut level_input).expect("failed to read_line");
    let level:u32 = level_input.trim().parse.().expect("enter a valid number");

    println!("Enter your cgpa");
   io::stdin().read_line(&mut cgpa_input).expect("failed to read_line");
    let level:f32 = cgpa_input.trim().parse.().expect("enter a valid number);
    	  
    	  println!("Are you a class rep");
    	 io::stdin().read_line(&mut level_input).expect("failed to read_line");
    	 let class_rep:bool  = input3.trim().parse.().expect("failed to input");


    println!("Enter name");
	io::stdin().read_line(&mut name).expect("failed to read_line");

	println!("Enter your email");
	io::stdin().read_line(&mut email).expect("not a valid string");

	println!("Enter your department");
	io::stdin().read_line(&mut department).expect("not a valid string");

	println!("Enter your state of origin");
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


    



