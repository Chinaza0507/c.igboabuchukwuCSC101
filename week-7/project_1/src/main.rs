use std::io;

fn main() {
    let mut input1 = String::new();
    let mut input2 = String::new();

    

    println!("T for teacher\nL for lawyer\nA for academic\nOA for office administrator");

    io::stdin().read_line(&mut input1).expect("failed to input");
    let profession: &str = input1.trim();

    println!("How many years of experience do you have?");
    io::stdin().read_line(&mut input2).expect("failed to input");
    let years_of_exp: f32 = input2.trim().parse().expect("failed to input");

    let office_admin_vector = vec![
        "Intern",
        "Administrator",
        "Senior Administrator",
        "Office Manager",
        "Director",
        "CEO",
    ];

    let academic = vec![
        "-",
        "Research assistant",
        "PhD candidate",
        "Post-doc researcher",
        "Senior lecturer",
        "Dean",
    ];

    let lawyer = vec![
        "Paralegal",
        "Junior associate",
        "Associate",
        "Senior associate 1-2",
        "Senior associate 3-4",
        "Partner",
    ];

    let teacher = vec![
        "Placement",
        "Classroom teacher",
        "Snr teacher",
        "Leading principal",
        "Deputy principal",
        "Principal",
    ];

    if profession == "OA" {
        println!("You are an office administrator");
        if years_of_exp >= 1.0 && years_of_exp <= 2.0 {
            println!("Your profession is {}", office_admin_vector[0]);
        } else if years_of_exp >= 3.0 && years_of_exp <= 5.0 {
            println!("Your professiom is {}", office_admin_vector[1]);
        } else if years_of_exp >= 5.0 && years_of_exp <= 8.0 {
            println!("Your positon is {}", office_admin_vector[2]);
        } else if years_of_exp >= 8.0 && years_of_exp <= 10.0 {
            println!("Your position is {}", office_admin_vector[3]);
        } else if years_of_exp >= 10.0 && years_of_exp <= 13.0 {
            println!("Your position is {}", office_admin_vector[4]);
        } else if years_of_exp > 13.0 {
            println!("Your profession is {}", office_admin_vector[5]);
        }
    } else if profession == "L" {
        println!("You are an Lawyer");
        if years_of_exp >= 1.0 && years_of_exp <= 2.0 {
            println!("Your profession is {}", lawyer[0]);
        } else if years_of_exp >= 3.0 && years_of_exp <= 5.0 {
            println!("Your professiom is {}", lawyer[1]);
        } else if years_of_exp >= 5.0 && years_of_exp <= 8.0 {
            println!("Your positon is {}", lawyer[2]);
        } else if years_of_exp >= 8.0 && years_of_exp <= 10.0 {
            println!("Your position is {}", lawyer[3]);
        } else if years_of_exp >= 10.0 && years_of_exp <= 13.0 {
            println!("Your position is {}", lawyer[4]);
        } else if years_of_exp > 13.0 {
            println!("Your profession is {}", lawyer[5]);
        }
    }
         else if profession == "A" {
        println!("You are an Academic");
        if years_of_exp >= 1.0 && years_of_exp <= 2.0 {
            println!("Your profession is {}",academic[0]);
        } else if years_of_exp >= 3.0 && years_of_exp <= 5.0 {
            println!("Your professiom is {}", academic[1]);
        } else if years_of_exp >= 5.0 && years_of_exp <= 8.0 {
            println!("Your positon is {}", academic[2]);
        } else if years_of_exp >= 8.0 && years_of_exp <= 10.0 {
            println!("Your position is {}", academic[3]);
        } else if years_of_exp >= 10.0 && years_of_exp <= 13.0 {
            println!("Your position is {}", academic[4]);
        } else if years_of_exp > 13.0 {
            println!("Your profession is {}", academic[5]);
}
}
    else if profession == "T" {
        println!("You are an Teacher");
        if years_of_exp >= 1.0 && years_of_exp <= 2.0 {
            println!("Your profession is {}",teacher[0]);
        } else if years_of_exp >= 3.0 && years_of_exp <= 5.0 {
            println!("Your professiom is {}", teacherC[1]);
        } else if years_of_exp >= 5.0 && years_of_exp <= 8.0 {
            println!("Your positon is {}", teacher[2]);
        } else if years_of_exp >= 8.0 && years_of_exp <= 10.0 {
            println!("Your position is {}", teacher[3]);
        } else if years_of_exp >= 10.0 && years_of_exp <= 13.0 {
            println!("Your position is {}", teacher[4]);
        } else if years_of_exp > 13.0 {
            println!("Your profession is {}", teacher[5]);
        }
    }else {
        println!("null or invalid profession");
    }
}
