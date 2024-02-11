use std::hash::{Hasher, BuildHasher};
use ahash::RandomState;
use crate::data::WordDict;

pub struct BloomFilter {
    array: Vec<bool>,
    hashers: [ahash::AHasher; 2], 
    pub optimal_len: u64,
    pub optimal_num_hashers: u32,
}

impl<'a> BloomFilter {
    pub fn build(word_list: &'a WordDict, accepted_error_rate: f32)-> BloomFilter {
        let data = word_list.get_data();
        //Optimal number formulas taken from: https://en.wikipedia.org/wiki/Bloom_filter
        let two: f32 = 2.0;
        let optimal_num_hashers = -(accepted_error_rate.ln()/two.ln()) as u32;
        let optimal_len = ((data.len() as f32)*(-accepted_error_rate.ln()/(two.ln()).powi(2))) as u64;
        
        let hasher_1 = RandomState::new().build_hasher();
        let hasher_2 = RandomState::new().build_hasher();
        let hashers = [hasher_1, hasher_2];
         
        let array: Vec<bool> = vec![false; optimal_len as usize];
        let mut bloomfilter = BloomFilter {
            array: array,
            hashers: hashers,
            optimal_len: optimal_len,
            optimal_num_hashers: optimal_num_hashers,
        };
        for word in data {
            let (hash_k, hash_m) = bloomfilter.hash_wrapper(word);
            for i in 0..(optimal_num_hashers as u64){
                let hash_i = i.wrapping_mul(hash_k.wrapping_add(hash_m));
                bloomfilter.array[(hash_i % optimal_len) as usize] = true;
            }
        }
        return bloomfilter;
    }

    pub fn in_filter(&self, word: &String) -> bool{
        let (hash_k, hash_m) = self.hash_wrapper(&word);

        //create i hashes by combining the two we have
        for i in 0..(self.optimal_num_hashers as u64){
            let hash_i = i.wrapping_mul(hash_k.wrapping_add(hash_m));
            let passes_hash = self.array[(hash_i % self.optimal_len) as usize];
            if !passes_hash{
                return false;
            }
        }
        return true;

    }
    //Wrapper since we cannot update internal state of hasher.
    fn hash_wrapper(&self, value: &String) -> (u64, u64) {
        let mut hash0 = self.hashers[0].clone();
        let mut hash1 = self.hashers[1].clone();
        hash1.write(value.as_bytes());
        hash0.write(value.as_bytes());
            
        let hash_m = hash1.finish();
        let hash_k = hash0.finish();
        return (hash_k, hash_m);
    }
}


