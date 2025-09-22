use std::collections::HashMap;

#[derive(Debug)]
struct Class {
    name: String,
    students: Vec<String>,
    grades: HashMap<String, Vec<f32>>,
}

impl Class {
    fn new(name: String) -> Self {
        Class {
            name,
            students: Vec::new(),
            grades: HashMap::new(),
        }
    }

    fn add_student(&mut self, student_name: String) {
        self.students.push(student_name.clone());
        self.grades.insert(student_name, Vec::new());
    }

    fn record_grade(&mut self, student_name: &str, grade: f32) {
        if let Some(grades) = self.grades.get_mut(student_name) {
            grades.push(grade);
        } else {
            println!("Student {} not found in class {}", student_name, self.name);
        }
    }

    fn average_grade(&self, student_name: &str) -> Option<f32> {
        self.grades.get(student_name).and_then(|grades| {
            if grades.is_empty() {
                None
            } else {
                Some(grades.iter().sum::<f32>() / grades.len() as f32)
            }
        })
    }

    fn display_grades(&self) {
        for (student, grades) in &self.grades {
            let avg = self.average_grade(student).unwrap_or(0.0);
            println!("{}: Grades: {:?}, Average: {:.2}", student, grades, avg);
        }
    }
}


fn main() {
    let mut class = Class::new("Math 101".to_string());
    class.add_student("Alice".to_string());
    class.add_student("Bob".to_string());
    class.record_grade("Alice", 85.0);
    class.record_grade("Alice", 90.0);
    class.record_grade("Bob", 78.0);
    class.record_grade("Bob", 82.0);
    class.display_grades();
}
