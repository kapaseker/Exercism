use std::collections::HashMap;

pub struct School(HashMap<String, u32>);

impl School {
    pub fn new() -> School {
        School(HashMap::new())
    }

    pub fn add(&mut self, grade: u32, student: &str) {
        let student = student.to_string();
        self.0.entry(student).or_insert(grade);
    }

    pub fn grades(&self) -> Vec<u32> {
        let mut vec: Vec<u32> = self.0.values().map(|s| *s).collect();
        vec.sort();
        vec.dedup();
        vec
    }
    
    pub fn grade(&self, grade: u32) -> Vec<String> {
        let mut students = vec![];
        for (name,grd) in self.0.iter() {
            if *grd == grade { 
                students.push(name.clone());
            }
        }
        students.sort();
        students
    }
}