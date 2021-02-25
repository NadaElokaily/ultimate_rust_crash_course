#[derive(Debug)]
struct Student{
    name: String
}

impl Student{
    fn new(name: String)->Student{
        Student{
            name: name
        }
    }
}

struct Course{
    name:String
}

impl Course{
    fn new(name: String)->Course{
        Course{
            name: name
        }
    }
    fn students(&self, platform: &Platform)->Vec<String>{
        platform.enrollments.iter()
        .filter(|e| e.course.name == self.name)
        .map(|e| {e.student.name.clone()})
        .collect()        
    }
}

struct Enrollment<'a>{
    student: &'a Student,
    course: &'a Course
}

impl <'a> Enrollment<'a>{
    fn new(student:&'a Student , course:&'a Course)->Enrollment<'a>{
        Enrollment{
            student: student,
            course: course
        }
    }
}

struct Platform<'a>{
    enrollments: Vec<Enrollment<'a>>
}

impl <'a> Platform<'a>{
    fn new()->Platform<'a>{
        Platform{
            enrollments: Vec::new()
        }
    }
    fn enroll(&mut self, student: &'a Student, course:&'a Course){
        let enrollment = Enrollment::new(student, course);
        self.enrollments.push(enrollment);
    }
}

fn main() {
    let mut john = Student::new("John".to_string());
    let mut cat = Student::new("cat".to_string());
    let mut nada = Student::new("nada".to_string());
    let mut bio = Course::new("bio".to_string());
    let mut platform = Platform::new();
    platform.enroll(&john, &bio);
    platform.enroll(&cat, &bio);
    platform.enroll(&nada, &bio);
    println!("{:?}",bio.students(&platform));

}
