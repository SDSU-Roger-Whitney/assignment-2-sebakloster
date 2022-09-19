pub struct StudentGrades {
    name: String,
    grades: Vec<u32>,
}

impl StudentGrades{
    fn student_grades(name: String, grades: Vec<u32>) -> StudentGrades{
        StudentGrades {
            name,
            grades
        }
    }

    fn average(&self) -> f32 {
        let mut sum: u32 = 0;
        for x in &self.grades {
            sum = sum + x;
        }
        sum as f32 / self.grades.len() as f32
    }
    fn grade(&self) -> String {
        let avg: f32 = self.average();
        match avg.round() as u32 {
            0..=59 => String::from("F"),
            60..=69 => String::from("D"),
            70..=79 => String::from("C"),
            80..=89 => String::from("B"),
            90..=100 => String::from("A"), 
            _=> String::from("[error] out of range")
        }
    }

}

fn main(){

    let student1 = StudentGrades::student_grades(String::from("Sebastian"), vec![100, 90, 70]);
    println!("The avg score of {} is: {}", student1.name, student1.average());
    println!("The letter grade of {} is: {} ", student1.name, student1.grade());

    let student2 = StudentGrades::student_grades(String::from("Agustin"), vec![100, 90, 20]);
    println!("The avg score of {} is: {}", student2.name, student2.average());
    println!("The letter grade of {} is: {} ", student2.name, student2.grade());

    //PROBLEM 1 TESTS
    assert_eq!(student1.name, "Sebastian");
    assert_eq!(student1.average(), 86.666664);
    assert_eq!(student2.name, "Agustin");
    assert_eq!(student2.grade(), "C");

}