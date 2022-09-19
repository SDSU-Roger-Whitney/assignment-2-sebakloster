use std::fs::File;
use std::io::{prelude::*, BufReader};

#[derive(Debug)]
pub struct Course {
    catalog_number : String,
    title : String,
    instructor : String,
    time : u32,
    days : String 
}

#[derive(Debug)]
pub struct CourseSchedule {
    courses_schedule: Vec<Course>
}

impl CourseSchedule {

    fn from_file(file_path:String) -> CourseSchedule {
        let file = File::open(file_path).expect("file not found!");
        let reader = BufReader::new(file);
        let lines: Vec<String> = reader.lines().collect::<Result<_, _>>().unwrap();
        let mut course_schedules_temp: Vec<Course> = Vec::new();
        for line in lines.iter().skip(2) {
            let split = line.split("	");
            let mut catalog_number = String::new();
            match split.clone().nth(4){
                Some(i) => catalog_number = i.to_string(),
                None => catalog_number = String::from("Null Catalog #"),
              }

            let mut title = String::new();
            match split.clone().nth(5){
                Some(i) => title = i.to_string(),
                None => title = String::from("Null title"),
              }

            let mut instructor : String = String::new();
            match split.clone().nth(25){
              Some(i) => instructor = i.to_string(),
              None => instructor = String::from("Null instructor"),
            }

            let mut time: String = String::new();
            match split.clone().nth(22){
                Some(i) => time = i.to_string(),
                None => time = 0.to_string(),
            }

            let mut days: String = String::new();
            match split.clone().nth(24){
                Some(i) => days = String::from(i),
                None => days = String::from("No days defined"),
              }

            let course = Course {
                catalog_number,
                title,
                instructor,
                time: time.parse::<u32>().unwrap_or(0),
                days
            };
            course_schedules_temp.push(course);

        }
        CourseSchedule{
            courses_schedule : course_schedules_temp
        }
    }

    fn courses_at(&self, time: &str, days: &str) -> Vec<&Course>{
        let mut filtered_courses: Vec<&Course> = Vec::new();
        let u32_time = time.parse().unwrap();
        for course in &self.courses_schedule {
            if course.days.eq(days) && course.time == u32_time {
                filtered_courses.push(course); 
            }
        }
        filtered_courses
    }

}

fn main(){
    //Testing problem 3
    
    let file_path = String::from("./search.xls");
    let schedule: CourseSchedule = CourseSchedule::from_file(file_path);
    let filtered_courses = schedule.courses_at("1900","MW");

    println!("\n--------------------------------------------------------------\n");
    println!("Showing courses on MW at 19:00 =>\t\t");
    for y in filtered_courses {
            println!("{:?}", y);
    }

    println!("\n--------------------------------------------------------------\n");
    println!("Showing all courses in xls file =>");
    for x in schedule.courses_schedule {
        println!("{:?}", x);
    }

}


