use indextree::Arena;
use std::cmp;
use crate::data::WordDict;

pub struct BKTreeWords<'a>
{
    bk_tree: Arena<(&'a String,i8)>, //change this to a struct to have named fields
    pub dist_fn: fn(&str, &str) -> i8, 
    pub dist_max: i8
}

impl<'a> BKTreeWords<'a>
{
    pub fn build(word_list: &'a WordDict, dist_fn: fn(&str, &str) -> i8) -> Self 
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
                let new_node = bk_tree.new_node((word, dist)); 
                dist_max = cmp::max(dist_max, dist); 
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
    pub fn find_correction(&self, word_to_check: &str) -> Option<String> {
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
