use std::rc::Rc;
use core::cell::RefCell;

#[derive(Debug)]
struct Student{
    name: String,
    courses: Vec<Rc<RefCell<Course>>>
}

impl Student{
    fn new(name: String)->Student{
        Student{
            name: name, 
            courses: Vec::new()
        }
    }
}

#[derive(Debug)]
struct Course {
    name: String,
    students: Vec<Rc<RefCell<Student>>>
}

impl Course{
    fn new(name: String)->Course{
        Course{
            name: name, 
            students: Vec::new()
        }
    }
    fn add_student(
        student: Rc<RefCell<Student>>,        
        course: Rc<RefCell<Course>>
    ){
        student.borrow_mut().courses.push(course.clone());
        course.borrow_mut().students.push(student.clone());
    }
}

fn main() {
    let John = Rc::new(RefCell::new(Student::new("John".to_string())));
    let bio = Rc::new(RefCell::new(Course::new("bio".to_string())));
    let chem = Rc::new(RefCell::new(Course::new("chem".to_string())));
    Course::add_student(John.clone(), bio);
    Course::add_student(John.clone(), chem);
    // println!("{:?}",John.borrow_mut().courses);
    for c in John.borrow_mut().courses.iter() {
        println!("{}",c.borrow_mut().name);
    }
}
