use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::Iterator;

pub struct DictionaryCombinationGenerator {
    dictionary: Vec<String>,
    index: usize,
    state_file: String,
}

impl DictionaryCombinationGenerator {
    pub fn new(dictionary_file: &str, state_file: &str) -> Self {
        // Open the file and read all the words into a Vec<String>
        let file = File::open(dictionary_file).expect("failed to open dictionary file");
        let reader = BufReader::new(file);
        let dictionary: Vec<String> = reader
            .lines()
            .map(|line| line.expect("failed to read line"))
            .collect();

        // Try to recover the index from the state file
        let index = match File::open(&state_file) {
            Ok(file) => {
                let mut reader = BufReader::new(file);
                let mut line = String::new();
                reader
                    .read_line(&mut line)
                    .expect("failed to read from state file");
                line.trim().parse().unwrap_or(0)
            }
            Err(_) => 0,
        };

        Self {
            dictionary,
            index,
            state_file: state_file.to_string(),
        }
    }

    pub fn save_state(&self) {
        // Save the current index to the state file
        std::fs::write(&self.state_file, self.index.to_string()).expect("failed to write to state file");
    }
}

impl Iterator for DictionaryCombinationGenerator {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        let next_item = self.dictionary.get(self.index).cloned();
        if next_item.is_some() {
            self.index += 1;
        }
        next_item
    }
}