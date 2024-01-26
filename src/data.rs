use std::fs;
use std::io::Write;

pub struct WordDict {
    frequency_data: Vec<String>,
}

impl WordDict {
    fn new(frequency_data: Vec<String>) -> Self {
        Self { frequency_data }
    }

    pub fn load_data(data_path: &str) -> Result<WordDict, std::io::Error> {
        let contents = fs::read_to_string(data_path)?;
        let mut data = Vec::new();
        for line in contents.split("\n") {
            let data_string = String::from(line.split("\t").next().unwrap());
            if !data_string.is_empty() {
                data.push(data_string);
            }
        }
        let word_data = WordDict::new(data);
        Ok(word_data)
    }
    //Unnessecary getters and setters
    pub fn word_in_data(&self, word: String) -> bool {
        self.frequency_data.contains(&word)
    }

    pub fn get_data(&self) -> &Vec<String> {
        &self.frequency_data
    }
}
