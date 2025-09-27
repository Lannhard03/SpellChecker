use indextree::{Arena, NodeId};
use std::cmp;
use crate::data::WordDict;

//Note:
//Maybe it is worth to store the strings as u8[] in the BKtree
//and the convert when printing!


pub struct BKTreeWords<'a>
{
    bk_tree: Arena<(&'a String,i8)>,
    root_id: NodeId,
    pub dist_fn: fn(&str, &str) -> i8, 
    pub dist_max: i8
}

impl<'a> BKTreeWords<'a>
{
    pub fn build(word_list: &'a WordDict, dist_fn: fn(&str, &str) -> i8) -> Self 
    {
        let data = word_list.get_data();
        let mut bk_tree = Arena::with_capacity(data.len());
        let root = bk_tree.new_node((&data[0], 0));
        let mut dist_max = 0;
        let mut current_node;
        //Add each word to BK-tree structure (skip first node)
        for word in data.iter().skip(1) {
            current_node = root;
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
                dist_max = cmp::max(dist_max, dist); 
                current_node.append_value((word, dist), &mut bk_tree);
                break;
            } 
        }

         
        BKTreeWords { 
                    bk_tree: bk_tree,
                    root_id: root,
                    dist_fn: dist_fn,
                    dist_max: dist_max,
                    }              
    }
    pub fn find_correction(&self, word_to_check: &str) -> Option<String> {
        if self.bk_tree.is_empty() {
            return None;
        }
        //Take the root node of the bk_tree, guarenteed to exist.
        //Nodes to process is used as a stack, with the last element being
        //the top of the stack. 
        let mut nodes_to_process: Vec<_> = Vec::with_capacity(10);
        nodes_to_process.push(self.root_id);
        let mut best_word: &String = &String::from("");
        let mut best_dist = self.dist_max;
        while nodes_to_process.len() > 0 {
            //By above condition, there is atleast one element in the stack.
            let current_node_id = nodes_to_process.pop().unwrap();
            let current_node = &self.bk_tree[current_node_id]; //maybe use .get syntax here!!
            let current_dist = (self.dist_fn)(current_node.get().0, word_to_check);
            if current_dist < best_dist {
                (best_word, best_dist) = (current_node.get().0, current_dist);
            }

            let children = current_node_id.children(&self.bk_tree);
            
            for child in children {
                let current_child = &self.bk_tree[child]; //.get syntax??
                let diff = current_dist - current_child.get().1;
                let triangle_inequality = diff <= best_dist && diff >= -best_dist;
                if triangle_inequality {
                    nodes_to_process.push(child);
                }

            }

        }
        
        return Some(best_word.clone());
    }
}
