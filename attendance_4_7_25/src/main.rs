// Create struct Student(major)
struct Student {
    id: i32,
    major: String,
}
// Higher order function update major
fn update_major(class: &mut Vec<Student>, operation: fn(&mut Student, String)) {
    let major:String = "Computer Engineering".to_string();
    for stud in class.iter_mut() {
        operation(stud, major.clone());
    }
}
// First order fucntions assign_major(student, major_declared)
fn major_declared(s: &mut Student, major: String) {
    s.major = major;
}
// Display students in Vector given
fn print_values(items: &[Student]) {
    println!("Values: ");
    for item in items.iter() {
        println!("{} {}", item.id, item.major);
    }
    println!();
}

fn main() {
    // Create vector students 1,2,3 and update all majors
    let mut class = vec![
        Student {id: 111, major: "".to_string()},
        Student {id: 203, major: "".to_string()},
        Student {id: 551, major: "".to_string()},
    ];
    // Prior to change
    print_values(&class);
    update_major(&mut class, major_declared);
    // After change
    print_values(&class);
}

