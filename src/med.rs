use std::cmp::min;


pub fn lev_dist_opt(s1: &[u8], s2: &[u8]) -> i8 {
    let mut d1: Vec<usize> = vec![0; s2.len()+1]; 
    let mut d2: Vec<usize> = vec![0; s2.len()+1]; 
    

    for i in 0..s2.len()+1 {
        d1[i] = i;
    }

    for i in 0..s1.len() {
        d2[0] = i + 1;
        for j in 0..s2.len() {
            let deletion_cost = d1[j+1] + 1;
            let insertion_cost = d2[j] + 1;
            let substitution_cost = if s2[j] == s1[i] {
                                        d1[j]
                                    } else {
                                        d1[j] + 1
                                    };

            d2[j+1] = min(deletion_cost, min(insertion_cost, substitution_cost));
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
        let distance1 = lev_dist_opt("hej".as_bytes(), "hej".as_bytes());
        assert_eq!(distance1, 0);


        let distance2 = lev_dist_opt("".as_bytes(), "".as_bytes());
        assert_eq!(distance2, 0);


        let distance3 = lev_dist_opt("".as_bytes(), "hej".as_bytes());
        assert_eq!(distance3, 3);


        let distace4 = lev_dist_opt("koppar".as_bytes(), "kroppar".as_bytes());
        assert_eq!(distace4, 1);


        let distance5 = lev_dist_opt("kopparhej".as_bytes(), "kroppar".as_bytes());
        assert_eq!(distance5, 4)
    }
}
