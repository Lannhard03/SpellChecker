use indextree::{Arena, NodeId};
use std::cmp;
use crate::data::WordDict;


pub struct BKTreeWords<'a>
{
    bk_tree: Arena<(&'a [u8], i8)>,
    root_id: NodeId,
    pub dist_fn: fn(&[u8], &[u8]) -> i8, 
    pub dist_max: i8
}


impl<'a> BKTreeWords<'a>
{
    pub fn build(word_list: &'a WordDict, dist_fn: fn(&[u8], &[u8]) -> i8) -> Option<Self> 
    {
        if word_list.len() == 0 {
            return None;
        }

        let mut dict_bytes = word_list.get_data().iter()
                                             .map(|s| s.as_bytes());

        let mut bk_tree = Arena::with_capacity(word_list.len());
        let root = bk_tree.new_node((dict_bytes.next().unwrap(), 0));
        let mut dist_max = 0;
        let mut current_node;


        for word in dict_bytes {
            current_node = root;
            //Traverse tree to find where to add word.
            'traversal: loop {
                let dist = dist_fn(word, bk_tree[current_node].get().0);

                //Disregard duplicate words from the word list
                if dist == 0 {break;}

                let children = current_node.children(&bk_tree);

                //If child with same distance is found, go one level deeper in search
                for child in children {
                    let child_dist = bk_tree[child].get().1;
                    if child_dist == dist {
                        current_node = child;
                        continue 'traversal;
                    }
                }

                //If no child has the same distance, add a new child with that distance
                dist_max = cmp::max(dist_max, dist); 
                current_node.append_value((word, dist), &mut bk_tree);
                break;
            } 
        }

         
        Some(BKTreeWords {bk_tree, root_id : root, dist_fn, dist_max})            
    }


    pub fn find_correction(&self, word_to_check: &str) -> Option<String> {
        if self.bk_tree.is_empty() {
            return None;
        }


        //Nodes to process is used as a stack, with the last element being
        //the top of the stack. 
        let mut nodes_to_process: Vec<_> = Vec::with_capacity(10);

        //Take the root node of the bk_tree, guarenteed to exist.
        nodes_to_process.push(self.root_id);


        let mut best_word : &[u8] = "".as_bytes();
        let mut best_dist = self.dist_max;
        while nodes_to_process.len() > 0 {
            //By above condition, there is atleast one element in the stack.
            let current_node_id = nodes_to_process.pop().unwrap();
            let current_node = &self.bk_tree[current_node_id]; //maybe use .get syntax here!!
            let current_dist = (self.dist_fn)(current_node.get().0, word_to_check.as_bytes());


            if current_dist < best_dist {
                (best_word, best_dist) = (current_node.get().0, current_dist);
            }


            let children = current_node_id.children(&self.bk_tree);
            for child in children {
                let current_child = &self.bk_tree[child]; 
                let dist_diff = current_dist - current_child.get().1;

                let triangle_inequality = dist_diff <= best_dist && dist_diff >= -best_dist;
                if triangle_inequality {
                    nodes_to_process.push(child);
                }

            }

        }


        match String::from_utf8(best_word.to_vec()) {
            Ok(w) => return Some(w),
            Err(_) => return None,
        }
    }
}
