use std::cmp::min;
pub fn levenshtien_distance(s1: &str, s2: &str) -> i8 {
    //swap s1, s1 such that s2 is the longest
    let c1: Vec<char> = s1.chars().collect();
    let c2: Vec<char> = s2.chars().collect();
    let (c1, c2) = if c1.len() > c2.len() {
        (c2, c1)
    } else {
        (c1, c2)
    };

    let mut distances: Vec<u8> = vec![0; c1.len() + 1];
    let mut new_distances: Vec<u8> = Vec::with_capacity(distances.len());
    for (index2, char2) in c2.iter().enumerate() {
        new_distances.push(index2 as u8 + 1);
        for (index1, char1) in c1.iter().enumerate() {
            if char1 == char2 {
                new_distances.push(distances[index1])
            } else {
                new_distances.push(
                    1 + min(
                        distances[index1],
                        min(distances[index1 + 1], *new_distances.last().unwrap()),
                    ),
                )
            }
        }
        std::mem::swap(&mut distances, &mut new_distances);
        new_distances.clear();
    }
    *distances.last().unwrap() as i8
}

pub fn lev_dist_opt(s1: &[u8], s2: &[u8]) -> i8 {
    //We convert strings into arrays since we need to access
    //the same elements several times
    //let c1: Vec<u8> = s1.bytes().collect();
    //let c2: Vec<u8> = s2.bytes().collect();

    let mut d1: Vec<usize> = vec![0; s2.len()+1]; 
    let mut d2: Vec<usize> = vec![0; s2.len()+1]; 
    
    for i in 0..s2.len()+1 {
        d1[i] = i;
    }

    for i in 0..s1.len() {
        d2[0] = i + 1;
        for j in 0..s2.len() {
            let del_cost = d1[j+1] + 1;
            let ins_cost = d2[j] + 1;
            
            let sub_cost = if s2[j] == s1[i] {
                d1[j]
            } else {
                d1[j] + 1
            };

            d2[j+1] = min(del_cost, min(ins_cost, sub_cost));
        }
        std::mem::swap(&mut d1, &mut d2);
    }
    return d1[s2.len()] as i8;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn edit_distance_test() {
        let distance1 = levenshtien_distance("hej", "hej");
        assert_eq!(distance1, 0);
        let distance2 = levenshtien_distance("", "");
        assert_eq!(distance2, 0);
        let distance3 = levenshtien_distance("", "hej");
        assert_eq!(distance3, 3);
        let distace4 = levenshtien_distance("koppar", "kroppar");
        assert_eq!(distace4, 1);
        let distance5 = levenshtien_distance("kopparhej", "kroppar");
        assert_eq!(distance5, 4)
    }
}
