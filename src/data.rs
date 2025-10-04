use core::fmt;
use std::fs;
use unicode_segmentation::UnicodeSegmentation;


pub struct WordDict {
    frequency_data: Vec<String>,
}


pub struct Text {
    words: Vec<(usize, String)>,
}


pub struct SpellingError<'a> {
    original_word: &'a str,
    line_number: usize,
    recommended_correction: Option<String>,
}


impl<'a> SpellingError<'a> {
    pub fn new(original_word: &'a str, line_number: usize,
               recommended_correction: Option<String>) -> Self {
        SpellingError {original_word, line_number, recommended_correction}
    }
}


impl fmt::Display for SpellingError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.recommended_correction {
            Some(word) => write!(f, "Incorrect Word on line {}: {}, Suggested correction: {}", self.line_number,
                                 self.original_word, word),
            None => write!(f, "Found no correction for word: {}", self.original_word) 
        }
    }
}


impl WordDict {
    fn new(frequency_data: Vec<String>) -> Self {
        Self { frequency_data }
    }


    pub fn load_data(data_path: &str) -> Result<WordDict, std::io::Error> {
        let dict_text = fs::read_to_string(data_path)?;
        let dict_data = dict_text.lines()
                                 .filter_map(|line| {
                                    line.split("\t").next()
                                 })
                                 .map(|word| {
                                    String::from(word)
                                 }).collect();


        let word_data = WordDict::new(dict_data);
        Ok(word_data)
    }


    pub fn word_in_data(&self, word: String) -> bool {
        self.frequency_data.contains(&word)
    }


    pub fn get_data(&self) -> &Vec<String> {
        &self.frequency_data
    }


    pub fn len(&self) -> usize {
        self.frequency_data.len()
    }
}


impl Text {
    pub fn load_text(text_path: &str) -> Result<Text, std::io::Error> {
        let text = fs::read_to_string(text_path)?;
        let lines = text.lines();

        let data: Vec<(usize, String)> = {
            lines.enumerate()
                 .flat_map(|(line_num, line)| {
                    line.unicode_words()
                        .map( |word| {
                            (line_num, String::from(word.to_lowercase()))
                        })
                        .collect::<Vec<(usize, String)>>()
                 }).collect()
        };


        Ok(Text{ words: data})
    }


    pub fn get_text(&self) -> &Vec<(usize, String)> {
        &self.words
    }
}
