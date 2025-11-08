use std::fs::File;
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
    data_path: String,
    ///Enter path of text to spellcheck here:
    #[arg(short, long, num_args = 1..)]
    text_path: Vec<String>,
    ///How likely that a incorrect word is labeled as correct
    #[arg(short, long, default_value_t = 0.01)]
    error_rate: f32,
    ///Wheter to force a rebuild of the spellchecker
    #[arg(short, long, default_value_t = false)]
    rebuild_spellchecker: bool,
    #[arg(short, long, default_value_t = true)]
    no_serialize: bool,
}


pub struct Config {
    texts: Vec<Text>, 
    dict: WordDict, 
    error_rate: f32,
    rebuild_spellchecker: bool,
    no_serialize: bool,
}


impl Config {
     pub fn build(args: Args) -> Result<Config, &'static str> {
        let data_path: String = args.data_path;
        let dict = match WordDict::load_data(&data_path){
            Ok(data) => data,
            Err(_) => {return Err("Couldn't read data path")}
        };

        let mut texts = Vec::new();
        let text_paths: Vec<String> = args.text_path;
        for text_path in text_paths {
            let text = match Text::load_text(&text_path) {
                Ok(data) => data,
                Err(_) => {return Err("Couldn't read text path")}
            };
            texts.push(text);
        } 


        Ok(Config {
            texts,
            dict,
            error_rate: args.error_rate,
            rebuild_spellchecker: args.rebuild_spellchecker,
            no_serialize: args.no_serialize,
        })
    }

    pub fn build_spellchecker(&self) -> Result<SpellChecker, String> {
        let bloom_filter = BloomFilter::build(&self.dict, self.error_rate);
        //Bloom filter build cannot fail.
        println!("Lenght of bloom_filter is: {}, and lenght of dictionary is: {}", 
            bloom_filter.optimal_len, self.dict.get_data().len());
        println!("Number of hashes is: {}", bloom_filter.optimal_num_hashers);


        let bk_tree;
        match BKTreeWords::build(&self.dict, med::lev_dist_opt) {
            Some(r) => bk_tree = r,
            None => {
                return Err(String::from("Couldn't create BKtree. Provided dictionary might be empty"));
            }
        }
        println!("Maximum distance: {}", bk_tree.dist_max);

        let spellchecker = SpellChecker::new(bk_tree, bloom_filter);
        if !self.no_serialize {
            match self.serialize_spellchecker(&spellchecker) {
                Err(str) => {println!("{}", str);}
                _ => () 
            }
        }

        return Ok(spellchecker);
    }

    pub fn deserialize_spellchecker(&self) -> Result<SpellChecker, String> {
        let mut serialized_checker;
        match File::open("spellcheckers/serialized_checker.bin") {
            Ok(file) => {serialized_checker = file},
            Err(_) => {

                println!("Could not read serialized spellchecker. Attempting build.");
                return self.build_spellchecker();
            }
        }

        let spell_checker: SpellChecker;
        match bincode::decode_from_std_read(&mut serialized_checker, bincode::config::legacy()) {
            Ok(checker) => {spell_checker = checker},
            Err(_) => {return Err(String::from("Could not deserialize spellchecker read from file."))} 
        }; 

        return Ok(spell_checker);
    }

    pub fn serialize_spellchecker(&self, spellchecker: &SpellChecker) -> Result<(), String> {
        let mut file;
        match File::create("spellcheckers/serialized_checker.bin") {
            Err(_) => {return Err(String::from("Could not open file to serialze"))}
            Ok(f) => file = f,
        }


        match bincode::encode_into_std_write(spellchecker, &mut file, bincode::config::legacy()) {
            Err(_) => {return Err(String::from("Could not serialize spellchecker."))}
            _ => ()
        }


        return Ok(());
    }
    
    pub fn run(&self) -> Result<(), String>{
        let now = Instant::now();

        let spell_checker;
        if !self.rebuild_spellchecker && !self.no_serialize  {
            match self.deserialize_spellchecker() {
                Ok(checker) => {spell_checker = checker;},
                Err(e) => {return Err(e)},
            }
        } else {
            match self.build_spellchecker() {
                Ok(checker) => {spell_checker = checker;},
                Err(e) => {return Err(e)},
            }
        } 

        for text in &self.texts {
            let spelling_errors = spell_checker.spell_check_text(&text);
            let report_name = format!("report_{}", text.get_name());

            println!("{}", match SpellChecker::create_report(&report_name, &spelling_errors) {
                Ok(_) => "Created report",
                Err(_) => "Error while creating report",
            });

        }

        println!("It took: {}", now.elapsed().as_secs_f32());

        return Ok(());
    }
}

fn main() {
    let args = Args::parse();
    let config = match Config::build(args){
        Ok(config) => config,
        Err(e) => {print!("{}", e); return}
    }; 


    match config.run() {
        Ok(_) => {return},
        Err(e) => {print!("{}", e); return}
    }
}


