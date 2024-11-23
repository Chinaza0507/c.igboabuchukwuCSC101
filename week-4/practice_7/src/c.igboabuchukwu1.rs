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