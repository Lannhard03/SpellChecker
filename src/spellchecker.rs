use std::fs;
use std::io::Write;
use crate::bktree::BKTreeWords;
use crate::bloomfilter::BloomFilter;
use crate::data::{Text, SpellingError};
use rayon::prelude::*;
use bincode::{Encode, Decode};

#[derive(Encode, Decode)]
pub struct SpellChecker {
    bk_tree: BKTreeWords,
    bloom_filter: BloomFilter,
}


impl<'a> SpellChecker {
    pub fn new(bk_tree: BKTreeWords, bloom_filter: BloomFilter) -> Self {
        SpellChecker {bk_tree, bloom_filter}
    }


    pub fn spell_check_text(&self, text: &'a Text) -> Vec<SpellingError<'a>> {
        text.get_text().into_par_iter()
                  .filter(|(_line, word)| !self.bloom_filter.in_filter(&word))
                  .map(|(line, word)| (word, line, self.bk_tree.find_correction(&word)))
                  .map(|word_tuple| SpellingError::new(&word_tuple.0, *word_tuple.1, word_tuple.2))
                  .collect()
    }
    

    pub fn create_report(report_name: &String, spelling_errors: &Vec<SpellingError>)
                         ->  Result<(), std::io::Error> {
        println!("Creating report for file: {}", report_name);


        let file_creation = fs::File::create(format!("./reports/report_{}", report_name));
        let mut report_file;
        match file_creation {
            Err(_) => {
                println!("Could not find report folder. Creating report folder.");
                fs::create_dir("./reports")?;
                return SpellChecker::create_report(report_name, spelling_errors);
            }
            Ok(file) => {report_file = file;}
        }


        let header = format!("Spell check for text in {}\n", report_name);
        report_file.write_all(&header[..].as_bytes())?;


        for spell_error in spelling_errors {
            report_file.write_all(&spell_error.to_string().as_bytes())?;
            report_file.write_all(b"\n")?;
        }


        Ok(())
    }
}


