#![allow(dead_code)]

pub struct Options {
    pub path: String, // cashcode path, example: COM1
    pub baud_rate: u32,
}

impl Default for Options {
    fn default() -> Self {
        Self { path: "".to_lowercase(), baud_rate: 9600 }
    }
}