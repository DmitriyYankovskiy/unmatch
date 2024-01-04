use std::sync::Mutex;

#[derive(Debug)]
pub struct Game {
    pub count: Mutex<usize>,
}

impl Game {
    pub fn add(& self) {
        let mut count = self.count.lock().unwrap();
        *count += 1;
    }
    
    pub fn get(&self) -> usize {
        let count = self.count.lock().unwrap();
        *count
    }
}