//! Simulate File
#![allow(unused_variables)]
#![allow(dead_code)]

use std::fmt;
use std::fmt::{Display};

#[derive(Debug, PartialEq)]
pub enum FileState {
    Open,
    Closed,
}

/// file means accesible file
#[derive(Debug)]
pub struct File {
    name: String,
    data: Vec<u8>,
    state: FileState,
}

trait Read {
    fn read(self: &Self, save_to: &mut Vec<u8>) -> Result<usize, String>;
}

impl Display for FileState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            FileState::Open => write!(f, "OPEN"),
            FileState::Closed => write!(f, "CLOSED"),
        }
    }
}

impl Read for File {
    fn read(
        self: &File,
        save_to: &mut Vec<u8>
    ) -> Result<usize, String> {
        if self.state != FileState::Open {
            return Err(String::from("File must be open for reading"));
        }
        let mut tmp = self.data.clone();
        let read_length = tmp.len();
        save_to.reserve(read_length);
        save_to.append(&mut tmp);
        Ok(read_length)
    }
}

impl Display for File {
    fn fmt(&self, f: &mut fmt::Formatter,) -> fmt::Result {
        write!(f, "<{} {}>", self.name, self.state)
    }
}

impl File {
    /// new file consider as empty, but file name is required.
    /// 
    /// # examples
    /// ```
    /// let f = File::new("f1.txt");
    /// ```
    /// 
    pub fn new(name: &str) -> File {
        File {
            name: String::from(name),
            data: Vec::new(),
            state: FileState::Closed,
        }
    }

    pub fn new_with_data(name: &str, data: &Vec<u8>) -> File {
        let mut f = File::new(name);
        f.data = data.clone();
        f
    }

    /// return file size(byte).
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// return file name.
    pub fn name(&self) -> String {
        self.name.clone()
    }
}

fn open(mut f: File) -> Result<File, String> {
    f.state = FileState::Open;
    Ok(f)
}

fn close(mut f: File) -> Result<File, String> {
    f.state = FileState::Closed;
    Ok(f)
}

fn main() {
    let f5 = File::new("5.txt");

    println!("{:?}", f5);
    println!("{}", f5);
}