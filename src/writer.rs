use std::fs::{File, OpenOptions};
use std::io::{self, Write};

pub struct Writer {
    buffer_size: usize,
    file: File,
}

impl Writer {
    pub fn new(buffer_size: usize) -> Self {
        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open("data.glg")
            .expect("Unable to create/open file");

        Self { buffer_size, file }
    }

    pub fn append(&mut self, buffer: &[[u8; 64]; 64], label: &str) {
        let mut entry = String::new();
        entry.push_str(&format!("{}: ", label.chars().next().unwrap()));
        entry.push_str(&Self::flatten_to_string(buffer));
        entry.push('\n');

        self.file
            .write_all(entry.as_bytes())
            .expect("Failed to write to file");
    }

    fn flatten_to_string(buffer: &[[u8; 64]; 64]) -> String {
        buffer
            .iter()
            .flat_map(|row| row.iter())
            .map(|&b| if b == 0 { "0" } else { "1" })
            .collect::<Vec<_>>()
            .join("")
    }
}
