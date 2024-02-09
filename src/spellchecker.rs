use std::fs;
use std::io::Write;
use crate::bktree::BKTreeWords;
use crate::bloomfilter::BloomFilter;
use crate::data::{Text, SpellingError};
use rayon::prelude::*;

pub struct SpellChecker<'a> {
    bk_tree: BKTreeWords<'a>,
    bloom_filter: BloomFilter,
}
impl<'a> SpellChecker<'a> {
    pub fn new(bk_tree: BKTreeWords<'a>, bloom_filter: BloomFilter) -> Self {
        SpellChecker { bk_tree: bk_tree, bloom_filter: bloom_filter }
    }
    pub fn spell_check_text(&self, text: &'a Text) -> Vec<SpellingError<'a>> {
        text.get_text().into_par_iter()
                  .filter(|word| !self.bloom_filter.in_filter(&word))
                  .map(|word| (word, self.bk_tree.find_correction(&word)))
                  .map(|word_tuple| SpellingError::new(&word_tuple.0, word_tuple.1))
                  .collect()
    }
    
    pub fn create_report(report_name: &String, errors: &Vec<SpellingError>) ->  Result<(), std::io::Error> {
        println!("Creating report in file: {}", report_name);
        let mut report_file = fs::File::create(format!("./reports/{}", report_name))?;
        let header = format!("Spell check for text in {}", report_name);
        report_file.write_all(&header[..].as_bytes())?;
        report_file.write_all(b"\n")?;
        for spell_error in errors {
            report_file.write_all(&spell_error.to_string().as_bytes())?;
            report_file.write_all(b"\n")?;
        }
        Ok(())
    }
}


