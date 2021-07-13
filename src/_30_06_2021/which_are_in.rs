// Given two arrays of strings a1 and a2 return a sorted
// array r in lexicographical order of the strings of a1
// which are substrings of strings of a2.

use lexical_sort::{natural_lexical_cmp, StringSort};

pub fn in_array(arr_a: &[&str], arr_b: &[&str]) -> Vec<String> {
    let mut res: Vec<String> = Vec::new();
    for item_a in arr_a.iter() {
        'l: for item_b in arr_b.iter() {
            if item_b.contains(item_a) {
                res.push(String::from(*item_a));
                break 'l;
            }
        }
    }
    res.string_sort(natural_lexical_cmp);
    res
}

use std::cmp::Ordering;

#[allow(dead_code)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd)]
struct Wrapper<T: Eq + PartialOrd>(T);

impl<T: Copy + Eq + PartialOrd> HasLength<T> for Wrapper<T> {
    fn get(&self) -> T {
        self.0
    }

    fn len(&self) -> usize {
        self.0.len()
    }
}

trait HasLength<T: ?Sized> {
    fn len(&self) -> usize;
    fn get(&self) -> T;
}

impl Ord for Wrapper<String> {
    fn cmp(&self, other: &Self) -> Ordering {
        let s: String = self.get();
        let o: String = other.get();
        for i in 0..std::cmp::max(s.len(), o.len()) {
            println!("{:?}")
        }
        Ordering::Equal
    }
}
