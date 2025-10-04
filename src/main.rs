use std::time::Instant;
use crate::spellchecker::SpellChecker;
use crate::bktree::BKTreeWords;
use crate::bloomfilter::BloomFilter;
use crate::data::{WordDict, Text};
use clap::Parser;


pub mod med;
pub mod data;
pub mod spellchecker;
pub mod bktree;
pub mod bloomfilter;


#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    ///Enter path of dictionary file here:
    #[arg(short, long)]
    data_path: Option<String>,
    ///Enter path of text to spellcheck here:
    #[arg(short, long)]
    text_path: Option<String>,
    ///How likely that a incorrect word is labeled as correct
    #[arg(short, long, default_value_t = 0.01)]
    error_rate: f32,
    ///Wheter to force a rebuild of the spellchecker
    #[arg(short, long, default_value_t = false)]
    rebuild_spellchecker: bool,
}


pub struct Config {
    text: Text, 
    data: WordDict, 
    error_rate: f32
}


impl Config {
     pub fn build(args: Args) -> Result<Config, &'static str> {
        let data_path: String = match args.data_path {
            Some(arg) => arg,
            None=> {return Err("No data path provided")}
        };


        let data = match WordDict::load_data(&data_path){
            Ok(data) => data,
            Err(_) => {return Err("Couldn't read data path")}
        };


        let text_path: String = match args.text_path {
            Some(arg) => arg,
            None=> {return Err("No text path provided")}
        };


        let text = match Text::load_text(&text_path) {
            Ok(data) => data,
            Err(_) => {return Err("Couldn't read text path")}
        };


        Ok(Config {
            text,
            data,
            error_rate: args.error_rate,
        })
    }
    
    pub fn run(&self) {
        let now = Instant::now();


        let bloom_filter = BloomFilter::build(&self.data, self.error_rate);
        println!("Lenght of bloom_filter is: {}, and lenght of dictionary is: {}", 
                 bloom_filter.optimal_len, self.data.get_data().len());
        println!("Number of hashes is: {}", bloom_filter.optimal_num_hashers);


        let bk_tree;
        match BKTreeWords::build(&self.data, med::lev_dist_opt) {
            Some(r) => bk_tree = r,
            None => {
                println!("Couldn't create BKtree. Provided dictionay might be empty");
                return       
            }
        }
        println!("Maximum distance: {}", bk_tree.dist_max);


        let spell_checker = SpellChecker::new(bk_tree, bloom_filter); 
        let spelling_errors = spell_checker.spell_check_text(&self.text);


        println!("It took: {}", now.elapsed().as_secs_f32());


        println!("{}", match SpellChecker::create_report(&String::from("text.txt"), &spelling_errors) {
            Ok(_) => "Created report",
            Err(_) => "Error while creating report",
        })
    }
}

fn main() {
    let args = Args::parse();
    let config = match Config::build(args){
        Ok(config) => Some(config),
        Err(e) => {print!("{}", e); None}
    }; 


    match config {
        Some(config) => {config.run()},
        None => return
    }
}


