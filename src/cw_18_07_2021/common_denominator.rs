use std::iter::Iterator;

#[allow(dead_code)]
type Fraction = (i64, i64);

#[allow(dead_code)]
fn convert_fracts(l: Vec<Fraction>) -> Vec<Fraction> {
    let numerators: Vec<i64> = l.iter().map(|i| i.0).collect(); 
    let denominators: Vec<i64> = l.iter().map(|i| i.1).collect();
    let lcm = least_common_multiple_of_vec(denominators.clone());
    let factor_vec = denominators.iter().map(|n| lcm / n).collect::<Vec<i64>>();
    let mut res: Vec<Fraction> = vec![];
    for i in 0..numerators.len() {
        let num = numerators[i] * factor_vec[i];
        res.push((num, lcm));
    }
    res
}

/// Function for calculating greatest common divisor of two numbers
#[allow(dead_code)]
fn greatest_common_divisor(a: i64, b: i64) -> i64 {
    if a == 0 {
        b
    } else {
        greatest_common_divisor(b % a, a)
    }
}

/// Function for calculating least common multiple of two numbers
#[allow(dead_code)]
fn least_common_multiple(a: i64, b: i64) -> i64 {
    (a * b) / greatest_common_divisor(a, b)
}

trait RemoveDuplicates {
    type Duplicate;
    fn remove_duplicates(&mut self);
}

#[derive(Clone, Copy)]
struct MyTuple(i64, i64);

use std::fmt;
impl fmt::Debug for MyTuple {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{},{}]", self.0, self.1)
    }
}

impl PartialEq for MyTuple {
    fn eq(&self, other: &MyTuple) -> bool {
        (self.0 == other.0) & (self.1 == other.1)
        ||
        (self.0 == other.1) & (self.1 == other.0)
    }
}
impl Eq for MyTuple {}

impl<T: PartialEq + Copy> RemoveDuplicates for Vec<T> {
    type Duplicate = T;
    fn remove_duplicates(&mut self) {
        let mut i: usize = 0;
        let mut buf = vec![];
        while i < self.len() {
            let v = self[i];
            if buf.contains(&v) {
                self.remove(i);
            } else {
                buf.push(v);
                i += 1;
            }
        }        
    }
}

// use std::collections::HashSet;
// use std::collections::hash_map::RandomState;
// use std::iter::FromIterator;
use std::cmp::Eq;
use std::cmp::PartialEq;
// use std::hash::Hash;

#[allow(dead_code)]
fn cross_p(v: Vec<i64>, u: Vec<i64>) -> Vec<MyTuple> {
    let mut buf = vec![];
    for i in 0..v.len() {
        for j in 0..u.len() {
            let value = MyTuple(v[i], u[j]);
            if (j != i) & (!buf.contains(&value)) {
                buf.push(value);
            }
        }
    }
    buf
}

#[allow(dead_code)]
fn cross_p_self(v: Vec<i64>) -> Vec<MyTuple> {
    cross_p(v.clone(), v.clone())
}

fn least_common_multiple_of_vec(v: Vec<i64>) -> i64 {
    let mut vec: Vec<i64> = v.clone();
    while vec.len() != 1 {
        vec = cross_p_self(vec).iter().map(|my| least_common_multiple(my.0, my.1)).collect();
        vec.remove_duplicates();
    }
    vec[0]
}

pub fn run() {
    let mut input_my_tuple = vec![MyTuple(69, 130), MyTuple(87, 1310), MyTuple(3, 4)];
    let input = input_my_tuple.iter().map(|m| (m.0, m.1)).collect::<Vec<(i64, i64)>>();
    let res = convert_fracts(input);
    println!("res: {:?}", res);
    // let l = least_common_multiple_of_vec(vec![130, 1310, 4]);
    // println!("l: {}", l);
    // let a = vec![130, 1310, 4].iter().map(|a| l / a).collect::<Vec<i64>>();
    // println!("a: {:?}", a);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn testing(a: Vec<Fraction>, b: Vec<Fraction>) {
        assert_eq!(convert_fracts(a), b);
    }

    #[test]
    fn basics_convert_fracts() {
        testing(vec![(69, 130), (87, 1310), (3, 4)], vec![(18078, 34060), (2262, 34060), (25545, 34060)]);
        testing(vec![(690, 1300), (87, 1310), (30, 40)], vec![(18078, 34060), (2262, 34060), (25545, 34060)]);
    }
}