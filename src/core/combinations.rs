use std::fs::OpenOptions;
use std::io::prelude::*;

pub struct CombinationGenerator {
    alphabet: Vec<char>,
    length: usize,
    counter: Vec<usize>,
    max_length: usize,
    state_file: String,
}

impl CombinationGenerator {
    pub fn new(alphabet: &str, max_length: usize, state_file: &str) -> Self {
        // Initialize with empty counter
        let mut counter = vec![0; max_length];

        // Initialize from state_file if it exists and is not empty
        if let Ok(mut file) = OpenOptions::new().read(true).open(state_file) {
            let mut contents = String::new();
            if let Ok(_) = file.read_to_string(&mut contents) {
                if !contents.is_empty() {
                    // Assuming that the file contains a comma-separated list of indices
                    let saved_counter: Vec<usize> = contents.trim()
                        .split(',')
                        .map(|x| x.parse().unwrap())
                        .collect();
                    counter = saved_counter;
                }
            }
        }

        CombinationGenerator {
            alphabet: alphabet.chars().collect(),
            length: 1,
            counter,
            max_length,
            state_file: state_file.to_string(),
        }
    }

    pub fn next(&mut self) -> Option<String> {
        if self.length > self.max_length {
            return None;
        }

        let mut combination: String = self.counter.iter()
            .take(self.length)
            .map(|&index| self.alphabet[index])
            .collect();

        self.increment_counter();

        Some(combination)
    }

    fn increment_counter(&mut self) {
        for i in 0..self.length {
            self.counter[i] += 1;
            if self.counter[i] < self.alphabet.len() {
                return;
            } else {
                self.counter[i] = 0;
            }
        }
        self.length += 1;
    }

    pub fn save_state(&self) {
        let mut file = OpenOptions::new().write(true).create(true).open(&self.state_file).expect("failed to open state file");
        let counter_string = self.counter.iter().take(self.length).map(|x| x.to_string()).collect::<Vec<String>>().join(",");
        writeln!(file, "{}", counter_string).expect("failed to write to state file");
    }
}