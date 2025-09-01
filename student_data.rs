struct Student {
    name: String,
    age: u8,
    grade: char,
}

impl Student {
    fn new(name: &str, age: u8, grade: char) -> Student {
        Student {
            name: name.to_string(),
            age,
            grade,
        }
    }

    fn is_passed(&self) -> bool {
        match self.grade {
            'A' | 'B' | 'C' => true,
            _ => false,
        }
    }

    fn print_details(&self) {
        println!("Name: {}, Age: {}, Grade: {}", self.name, self.age, self.grade);
    }
}

fn main() {
    let student1 = Student::new("Shamsudeen", 32, 'C');
    let student2 = Student {
        name: String::from("Samuel Joshue"),
        age: 23,
        grade: 'A',
    };

    student1.print_details();
    student2.print_details();

    println!("{} passed? {}", student1.name, student1.is_passed());
    println!("{} passed? {}", student2.name, student2.is_passed());
}
