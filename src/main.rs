use std::env;
use std::process;
use std::time::Instant;
use std::fs;
use indextree::Arena;
use indextree::NodeId;
use unicode_segmentation::UnicodeSegmentation;
use std::cmp;
use crate::spellchecker::SpellChecker;
use crate::bktree::BKTreeWords;
use crate::bloomfilter::BloomFilter;
use crate::data::{WordDict, Text, SpellingError};
use std::hash::{Hasher, BuildHasher};
use ahash::{AHasher, RandomState};

pub mod med;
pub mod data;
pub mod spellchecker;
pub mod bktree;
pub mod bloomfilter;

fn main() {
    let mut args = env::args();
    args.next();

    let data_path: String = match args.next() {
        Some(arg) => arg,
        None=> {println!("No data path provided");process::exit(1);}
    };
    println!("{}", data_path);
    let data = match WordDict::load_data(&data_path){
        Ok(data) => data,
        Err(e) => {println!("Couldn't read data path");process::exit(0);}
    };

    let text_path: String = match args.next() {
        Some(arg) => arg,
        None=> {println!("No text path provided");process::exit(1);}

    };

    let text = match Text::load_text(&text_path) {
        Ok(data) => data,
        Err(e) => {println!("Couldn't read data path");process::exit(0);}
    };

    let bloom_filter = BloomFilter::build(&data, 0.01);
    println!("Lenght of bloom_filter is: {}, and lenght of dictionary is: {}", bloom_filter.optimal_len, data.get_data().len());
    println!("number of hashes is: {}", bloom_filter.optimal_num_hashers);

    //for word in text.get_text() {
    //    println!("{} is in bloom_filter: {}", word, bloomfilter.in_filter(&word));
    //}

    let bk_tree = BKTreeWords::build(&data, med::levenshtien_distance);
    println!("Maximum distance: {}", bk_tree.dist_max);
    let now = Instant::now();
    let checker = SpellChecker::new(bk_tree, bloom_filter); 

    let spelling_errors = checker.spell_check_text(&text);
    //for error in spelling_errors {
    //   println!("{}", error);
    //}
    println!("It took: {}", now.elapsed().as_secs_f32());
    println!("{}", match SpellChecker::create_report(&String::from("report.txt"), &spelling_errors) {
        Ok(_) => "Created report",
        Err(_) => "Error while creating report",
    })
}


