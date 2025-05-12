use std::fs::{File, OpenOptions};
use std::io::{Write, Seek, SeekFrom};

pub struct Jsonifier {
    buffer_size: usize,
    first_entry: bool,
}

impl Jsonifier {
    pub fn new(buffer_size: usize) -> Self {
        Self {
            buffer_size,
            first_entry: true,
        }
    }

    pub fn init(&mut self) {
        let mut file = File::create("data.json").expect("Unable to create file");
        file.write_all(b"{\n  \"data\": [\n").expect("Failed to write init");
        self.first_entry = true;
    }

    pub fn append(&mut self, buffer: &[[u8; 64]; 64], label: &str) {
        let mut file = OpenOptions::new()
            .write(true)
            .append(true)
            .create(true)
            .open("data.json")
            .expect("Unable to open file");

        if !self.first_entry {
            file.write_all(b",\n").expect("Failed to write comma");
        }

        let mut entry = String::new();
        entry.push_str("    {\n");
        entry.push_str(&format!("      \"label\": \"{}\",\n", label));
        entry.push_str("      \"buffer\": [\n");

        for (y, row) in buffer.iter().enumerate() {
            entry.push_str("        [");
            for (x, val) in row.iter().enumerate() {
                entry.push_str(&val.to_string());
                if x < row.len() - 1 {
                    entry.push_str(", ");
                }
            }
            entry.push(']');
            if y < buffer.len() - 1 {
                entry.push(',');
            }
            entry.push('\n');
        }

        entry.push_str("      ]\n");
        entry.push_str("    }");

        file.write_all(entry.as_bytes()).expect("Failed to write entry");

        self.first_entry = false;
    }

    pub fn close(&self) {
        let mut file = OpenOptions::new()
            .write(true)
            .append(true)
            .create(true)
            .open("data.json")
            .expect("Unable to open file");

        file.write_all(b"\n  ]\n}").expect("Failed to write closing brace");
    }
}
