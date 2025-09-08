use std::collections::LinkedList;
use Error::{EmptyBuffer, FullBuffer};

pub struct CircularBuffer<T> {
    // We fake using T here, so the compiler does not complain that
    // "parameter `T` is never used". Delete when no longer needed.
    capacity: usize,
    items: LinkedList<T>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    EmptyBuffer,
    FullBuffer,
}

impl<T> CircularBuffer<T> {
    pub fn new(capacity: usize) -> Self {
        Self {
            capacity,
            items: LinkedList::new(),
        }
    }

    pub fn write(&mut self, _element: T) -> Result<(), Error> {
        if self.capacity == self.items.len() {
            return Err(FullBuffer);
        }
        self.items.push_back(_element);
        Result::Ok(())
    }

    pub fn read(&mut self) -> Result<T, Error> {
        if self.items.is_empty() { 
            return Err(EmptyBuffer);
        }
        Ok(self.items.pop_front().unwrap())
    }

    pub fn clear(&mut self) {
        self.items.clear()
    }

    pub fn overwrite(&mut self, _element: T) {
        if self.capacity == self.items.len() { 
            self.read().expect("Read Exception");
        }
        self.write(_element).expect("Write Exception");
    }
}