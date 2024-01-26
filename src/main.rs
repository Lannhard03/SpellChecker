use std::env;
use std::process;
use std::time::Instant;
use std::fs;
use indextree::Arena;
use indextree::NodeId;
use unicode_segmentation::UnicodeSegmentation;
use std::cmp;
use crate::data::WordDict;

pub mod med;
pub mod data;

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

    let bk_tree = BKTreeWords::build(&data, med::levenshtien_distance);
    println!("{}", bk_tree.dist_max);
    let correction = bk_tree.find_correction("hejj").unwrap();
    println!("{} rättstavades som {}", "hejj", correction);
    println!("Calculated distance: {}", (bk_tree.dist_fn)("hejj", &correction));
    //println!("{:?}", bk_tree.bk_tree);
}

//Functionality: 
//Add dictionary from text file. And build bloomfilter and BK_tree
//Load and spell check a text file.
struct BKTreeWords<'a, F>
where F: Fn(&str, &str) -> i8,
{
    bk_tree: Arena<(&'a String,i8)>, //change this to a struct to have named fields
    dist_fn: F, 
    dist_max: i8
}
impl<'a, F> BKTreeWords<'a, F>
where F: Fn(&str, &str) -> i8
{
    fn build(word_list: &'a WordDict, dist_fn: F) -> Self 
    {

        let data = word_list.get_data();
        let mut bk_tree = Arena::new();
        let root = bk_tree.new_node((&data[0], 0));
        let mut dist_max = 0;
        //Add each word to BK-tree structure
        for word in data.iter().skip(1) {
            let mut current_node = root;
            //Traverse tree to find where to add word.
            'traversal: loop {
                let dist = dist_fn(word, bk_tree[current_node].get().0);
                //Disregard duplicate words from the word list
                if dist == 0 {break;}

                let children = current_node.children(&bk_tree);
                //If child with same distance is found, go one level deeper in search
                for child in children {
                    let child_dist = bk_tree[child].get().1; //Second field is distance to parent
                    if child_dist == dist {current_node = child; continue 'traversal;}
                }
                //If no child has the same distance, add a new child with that distance
                //if word == "att" {println!("HEJ NODE: {}, {}. Child of: {}", word, dist, bk_tree[current_node].get().0)};
                let new_node = bk_tree.new_node((word, dist)); 
                dist_max = cmp::max(dist_max, dist); 
                //println!("New node: {}, {}; Child of: {}", word, dist, bk_tree[current_node].get().0);
                //println!("{}", dist_max);
                current_node.append(new_node, &mut bk_tree);
                break;
            } 
        }

         
        BKTreeWords { 
                    bk_tree: bk_tree,
                    dist_fn: dist_fn,
                    dist_max: dist_max,
                    }              
    }
    fn find_correction(&self, word_to_check: &str) -> Option<String> {
        if self.bk_tree.is_empty() {
            return None;
        }
        //Do you really need to create
        //iter of entire list
        let mut nodes_to_process: Vec<_> = self.bk_tree.iter().take(1).collect(); 
        let mut best_word: &String = &String::from("");
        let mut best_dist = self.dist_max;
        while nodes_to_process.len() > 0 {
            let current_node = nodes_to_process.pop().unwrap();
            let current_dist = (self.dist_fn)(current_node.get().0, word_to_check);
            if current_dist < best_dist {
                (best_word, best_dist) = (current_node.get().0, current_dist);
            }

            //if best_dist == 0 {println!("LESS GO")};

            let first_child = current_node.first_child();
            if first_child.is_none() {continue;}
            let mut current_child = &self.bk_tree[first_child.unwrap()];
            loop {
                if (current_dist-current_child.get().1).abs() <= best_dist {
                    
                    if current_child.get().0 == "och" {println!("Best dist: {}", best_dist)};
                    //println!("Added to nodes to search {}", current_child.get().0);
                    nodes_to_process.push(current_child);
                }
                match current_child.next_sibling() {
                    Some(child) => current_child = &self.bk_tree[child],
                    None => break,
                }
            }
        }
        
        return Some(best_word.clone());
    }
}
//Used 
struct BloomFilter {
//Filter: byte array
//Hashes: Fast hash functions
//Impl: Build filter, check for membership
}
//struct Dictionary<F> 
//where F: Fn(&str, &str) -> u8,
//{
//dict_bloomfilter: BloomFilter,
//words: BKTreeWords<F>,
//Impl: Build bloomfilter
//}
//Program should: 
//Take in path to .txt file
//Read contents to good format (Use Unicode-Segmentation crate?)
//Find misspellt words using fast method
//Spellcheck each missspellt word
