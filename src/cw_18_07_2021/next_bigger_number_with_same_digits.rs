mod bruteforce {
    #[allow(dead_code)]
    pub fn next_bigger_number(n: i64) -> i64 {
        let mut all_perm = vec_unique_elements(all_permutations(n));
        all_perm.sort();
        println!("{:?}", all_perm);
        let k = all_perm.len();
        for i in 0..k {
            if (all_perm[i] == n) & (i + 1 < k) {
                return all_perm[i + 1];
            }
        }
        -1
    }
    
    #[allow(dead_code)]
    pub fn all_permutations(n: i64) -> Vec<i64> {
        permutation(n.to_string().chars().collect::<Vec<char>>()).iter().map(|v| String::from_iter(v).parse::<i64>().unwrap()).collect::<Vec<i64>>()
    }
    
    use std::collections::HashSet;
    use std::collections::hash_map::RandomState;
    use std::iter::FromIterator;
    use std::cmp::Eq;
    use std::hash::Hash;
    
    #[allow(dead_code)]
    pub fn permutation<T: Copy>(v: Vec<T>) -> Vec<Vec<T>> {
        let k = v.len();
        if k < 2 {
            vec![v]
        } else {
            let mut l: Vec<Vec<T>> = Vec::new();
            for i in 0..k {
                let m: T = v[i];
                let mut before_m = v[0..i].to_vec();
                let mut after_m = v[i + 1..k].to_vec();
                before_m.append(&mut after_m);
                for p in permutation(before_m).iter_mut() {
                    let mut tmp: Vec<T> = p.clone();
                    tmp.push(m);
                    exchange(&mut tmp, 0, p.len());
                    l.push(tmp);
                }
            }
            l
        }
    }
    
    #[allow(dead_code)]
    pub fn vec_unique_elements<T: Eq + Hash + Copy>(v: Vec<T>) -> Vec<T> {
        // Vec -> HashSet 
        // Vec <- HashSet 
        HashSet::<T, RandomState>::from_iter(v.iter().cloned()).into_iter().collect::<Vec<T>>()
    } 
    
    #[allow(dead_code)]
    pub fn exchange<T: Copy>(vec: &mut Vec<T>, i: usize, j: usize) {
        let tmp = vec[i];
        vec[i] = vec[j];
        vec[j] = tmp;
    }
}

mod solution {
    // https://www.codewars.com/kata/55983863da40caa2c900004e/solutions
    #[allow(dead_code)]
    pub fn next_bigger_number(n: i64) -> i64 {
        // n = 918651
        // represent number as vector of chars in Big Endian notation
        // 1 5 6 8 1 9
        let mut digits = n.to_string().chars().rev().collect::<Vec<char>>();

        // start from the left and find digit which less then previous one
        // 1 5 6 8 [1] 9
        let i = match (0..digits.len()-1).position(|i| digits[i] > digits[i+1]) {
            Some(i) => i+1,
            None => return -1,
        };

        // find index 'j' of first digit from the left
        // which grater than current digit
        // 1 |5| 6 8 [1] 9
        let j = (0..i).position(|j| digits[j] > digits[i] ).unwrap();

        // swap current digit with previous smallest greater
        // 1 |1| 6 8 [5] 9
        digits.swap(j,i);

        // reverse sort left part
        // 8 6 1 1 [5] 9
        digits[0..i].sort_by_key(|&i| std::cmp::Reverse(i));

        // make string from vector then parse string as i64
        digits.into_iter().rev().collect::<String>().parse().unwrap()

    }
}

use std::iter::FromIterator;

#[allow(dead_code)]
fn next_bigger_number(_n: i64) -> i64 {
    let n_str = _n.to_string();
    if _n < 12 {
        -1
    } else {
        let chars = n_str.chars().collect::<Vec<char>>();
        let k = chars.len();
        for i in 2..=k {
            let i_last_digits_chars: Vec<char> = chars[k - i..k].to_vec();
            let num_last_i_digits: i64 = String::from_iter(i_last_digits_chars).parse::<i64>().unwrap();
            let mut permutations_of_last_digits: Vec<i64> = bruteforce::vec_unique_elements(bruteforce::all_permutations(num_last_i_digits));
            permutations_of_last_digits = permutations_of_last_digits.iter().filter(|v| v.to_string().len() == i).map(|v| *v).collect::<Vec<i64>>();
            permutations_of_last_digits.sort();
            // println!("[{}]\t{:?}: {:?}", i, num_last_i_digits, permutations_of_last_digits);
            let number_of_permutations = permutations_of_last_digits.len();
            if number_of_permutations < 2{
                continue;
            }
            for j in 0..number_of_permutations {
                if !(permutations_of_last_digits[j] == num_last_i_digits) {
                    continue;
                }
                if j + 1 >= number_of_permutations {
                    continue;
                }
                if k == number_of_permutations {
                        return permutations_of_last_digits[j + 1];
                } else {
                    return {
                        let mut str_res = String::from_iter(chars[0..k - i].iter());
                        // println!("\t\t{}", str_res);
                        str_res.push_str(&permutations_of_last_digits[j + 1].to_string());
                        str_res.parse::<i64>().unwrap_or(-1)
                    };
                }
            }
        }
        -1
    }
}


pub fn run() {
    let i: &[i64; 8] = &[12300, 54003, 12234, 513, 2017, 144, 513, 5275513];
    for j in i {
        println!("{}\t\t=>\t{}", j, solution::next_bigger_number(*j));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(21,  next_bigger_number(12));
        assert_eq!(531, next_bigger_number(513));
        assert_eq!(2071,next_bigger_number(2017));
        assert_eq!(441, next_bigger_number(414));
        assert_eq!(414, next_bigger_number(144));
    }
}
