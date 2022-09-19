use std::fs::File;
use std::io::{prelude::*, BufReader};

//PROBLEM 1

#[derive(Debug)]
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

// -- Problem 2 --

pub struct CourseGrades {
    student_grades: Vec<StudentGrades>,
}

impl CourseGrades {
    fn from_file(file_path:String) -> CourseGrades {
        let file = File::open(file_path).expect("file not found!");
        let reader = BufReader::new(file);
        let lines: Vec<String> = reader.lines().collect::<Result<_, _>>().unwrap();
        let mut course_grades: Vec<StudentGrades> = Vec::new();
        for line in lines {
            let split = line.split(",");
            let mut counter = 0;
            let mut student_name = "";
            let mut grades = Vec::new();

            for s in split {
                if counter == 0 {
                    student_name = s;
                }else{
                   let grade = s.trim().to_string().parse::<f32>().unwrap() as u32;
                   grades.push(grade);
                }
                counter+=1;
            }
            let student = StudentGrades::student_grades(student_name.to_string(), grades);
            course_grades.push(student);
        }
        CourseGrades{
            student_grades : course_grades
        }
    }

    fn average(&self, mut grade_event: usize) -> Option<f32> {
        let mut n_grades_sum : u32 = 0;
        let mut grades_counter : u32 = 0; 
        grade_event -= 1;
        for student_grade in &self.student_grades {
            if grade_event > student_grade.grades.len() {
                return None
            }else{
                n_grades_sum += student_grade.grades[grade_event];
                grades_counter += 1;
            }
        }
        Some(n_grades_sum as f32 / grades_counter as f32)
    }

    fn student(&self, student_name:String) -> Option<&StudentGrades> {
        for student_grade in &self.student_grades {
            if student_grade.name.eq(&student_name){
                return Some(student_grade)
            }
        }
        None
    }

}

fn main(){

    let student1 = StudentGrades::student_grades(String::from("Sebastian"), vec![100, 90, 70]);
    println!("The avg score of {} is: {}", student1.name, student1.average());
    println!("The letter grade of {} is: {} ", student1.name, student1.grade());

    // PROBLEM 2
    let file_path = String::from("./file.txt");
    let course_grades1 = CourseGrades::from_file(file_path);

    println!("{:?}",course_grades1.student_grades);

    println!("Average: {:?}",  course_grades1.average(2).unwrap());
    println!("Student grades: {:?}", course_grades1.student(String::from("pete")));

    // PROBLEM 2 TESTS
    assert_eq!(course_grades1.average(3).unwrap(),76.5);
    assert_eq!(course_grades1.student(String::from("roger")).unwrap().name, "roger");

}