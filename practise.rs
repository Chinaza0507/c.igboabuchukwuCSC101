use std::io;
fn main {
	let mut input 1=String::new();
	let mut input 2 =String::new();

	println!("What is your profession");
	println!("pick one from D for designer,E for enginner,T for typist");
	io::stdin read_line(&mut input 1).expect(failed to read input);
	let profession:&str = input1.trim();

	println!("How many years of experience do you havee");
	io::stdin read_line(&mut input2).expect("failed to read line");
	let years_of_experience:f32 = input2.trim().parse().expect("failed to read line");

	 let teacher_vector = Vec![
"Placement",
        "Classroom teacher",
        "Snr teacher",
        "Leading principal",
        ];
        Let designer_vector = Vec![
        "Traniee"
        "Junior designer"
        "senior designer"
        ];
        let engineer_vector = Vec![
"Intern"
"Junior enginner"
"senior enginner"

        ];
        if profession =="D"{
        	println!("YOU are a designer");
        }
         if years_of_experience >=1.0 &&  years_of_experience<=2.0{
         println!("your profession is {}",designer_vector[0]);
         }else if years_of_experience >=2.0 && years_of_experience <=4.0{
         	println!("your profession is {}",designer_vector[1]);
         }else if years_of_experience >=4.0 && years_of_experience <+=6.0 {
         	println!("your profession is {}", designer_vector[2]);
         }
 if profession =="T"{
        	println!("YOU are a teacher");
        }
         if years_of_experience >=1.0 &&  years_of_experience<=2.0{
         println!("your profession is {}",teacher_vector[0]);
         }else if years_of_experience >=2.0 && years_of_experience <=4.0{
         	println!("your profession is {}",teacher_vector[1]);
         }else if years_of_experience >=4.0 && years_of_experience <+=6.0 {
         	println!("your profession is {}", teacher_vector[2]);
         }        
 
 

}
