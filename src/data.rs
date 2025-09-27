use core::fmt;
use std::fs;
use unicode_segmentation::UnicodeSegmentation;

pub struct WordDict {
    frequency_data: Vec<String>,
}

pub struct Text {
    words: Vec<String>,
}

pub struct SpellingError<'a> {
    original_word: &'a str,
    recommended_correction: Option<String>,
}

impl<'a> SpellingError<'a> {
    pub fn new(original_word: &'a str, recommended_correction: Option<String>) -> Self {
        SpellingError {
            original_word: original_word, 
            recommended_correction: recommended_correction, 
        }
    }
}

impl fmt::Display for SpellingError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.recommended_correction.clone() {
            Some(word) => write!(f, "Incorrect Word: {}, Suggested correction: {}", self.original_word, word),
            None => write!(f, "Found no correction for word: {}", self.original_word), 
        }
    }
}

impl WordDict {
    fn new(frequency_data: Vec<String>) -> Self {
        Self { frequency_data }
    }

    pub fn load_data(data_path: &str) -> Result<WordDict, std::io::Error> {
        let contents = fs::read_to_string(data_path)?;
        let lines = contents.split("\n");
        let mut data = Vec::with_capacity(lines.count());
        for line in contents.split("\n") {
            //Here we discard the frequency of the words, but the ordering by freq.
            //is used implicitly when BKtree is constructed.
            let data_string = String::from(line.split("\t").next().unwrap());
            if !data_string.is_empty() {
                data.push(data_string);
            }
        }
        let word_data = WordDict::new(data);
        Ok(word_data)
    }

    pub fn word_in_data(&self, word: String) -> bool {
        self.frequency_data.contains(&word)
    }

    pub fn get_data(&self) -> &Vec<String> {
        &self.frequency_data
    }
}

impl Text {
    pub fn load_text(text_path: &str) -> Result<Text, std::io::Error> {
        let contents = fs::read_to_string(text_path)?;
        let data = contents.unicode_words().map(|word| String::from(word.to_lowercase())).collect();
        Ok(Text{ words: data})
    }

    pub fn get_text(&self) -> &Vec<String> {
        &self.words
    }
}
