use std::io::Write;
fn main() {
    //  information on sutdent name, matric number, department, level
    let student_vec: Vec<(&str, &str, &str, u32)> = vec![
        ("Oluchi Mordi", "ACC10211111", "Accounting", 300),
        ("Adams Aliyu", "ECO10110101", "Economics", 100),
        ("Shania Bolade", "CSC10328828", "Computer", 200),
        ("Adekunle Gold", "EEE11020202", "Eletrical", 200),
        ("Blanca Edemoh", "MEE10202001", "Mechanical", 100),
    ];
    let mut information = String::new();
    for student in student_vec {
        let student_entry = format!("Name: {},Matric number :{}, department:{},Level :{}\n", student.0, student.1, student.2, student.3);
        information += &student_entry;
    }
    println!("{}", information);
    let mut file = std::fs::File::create("student.txt").expect("create failed");
    file.write_all(information.as_bytes()).unwrap();
}