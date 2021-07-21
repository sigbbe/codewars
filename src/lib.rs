

use std::collections::HashSet;
use std::collections::hash_map::RandomState;
use std::hash::Hash;
use std::iter::FromIterator;
use std::vec::Vec;

#[allow(dead_code)]
pub fn remove_duplicates<T: Eq + Hash + Copy>(v: Vec<T>) -> Vec<T> {
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

#[allow(dead_code)]
pub fn all_permutations(n: i64) -> Vec<i64> {
    permutation(n.to_string().chars().collect::<Vec<char>>()).iter().map(|v| String::from_iter(v).parse::<i64>().unwrap()).collect::<Vec<i64>>()
}

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

pub mod print_hello {
    use std::env::args;
    use std::env::current_dir;

    #[allow(dead_code)]
    pub fn get_file_name() {
        println!("{:?}", current_dir().unwrap());
        println!("{:?}", args());
    }
}