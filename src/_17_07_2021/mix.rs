
use std::collections::HashSet;
use std::collections::hash_map::RandomState;
use std::iter::FromIterator;
use std::iter;
use std::cmp::Ordering;

#[allow(dead_code)]
fn mix(s1: &str, s2: &str) -> String {
    let s1_chars = s1.chars().filter(|v| {
        v.is_ascii_lowercase() & v.is_alphabetic() 
    }).collect::<Vec<char>>();
    let s2_chars = s2.chars().filter(|v| {
        v.is_ascii_lowercase() & v.is_alphabetic() 
    }).collect::<Vec<char>>();
    let mut all_valid_chars = s1_chars.iter().cloned().collect::<Vec<char>>();
    all_valid_chars.extend(s2_chars.iter().cloned());
    let unique_valid_chars: HashSet<&char, RandomState> = HashSet::from_iter(all_valid_chars.iter());
    let mut res_vec: Vec<(char, char, usize)> = Vec::new();
    for unique_char in unique_valid_chars.into_iter() {
        let count_in_s1 = s1_chars.iter().filter(|&n| *n == *unique_char).count();
        let count_in_s2 = s2_chars.iter().filter(|&n| *n == *unique_char).count();
        let from_which_string_and_count: (char, usize) = {
            if count_in_s1 == count_in_s2 {
                ('=', count_in_s1)
            } else if count_in_s1 > count_in_s2 {
                ('1', count_in_s1)
            } else {
                ('2', count_in_s2)
            }
        };
        let el: (char, char, usize) = (from_which_string_and_count.0, *unique_char, from_which_string_and_count.1);
        res_vec.push(el);
    }
    if res_vec.len() < 1 {
        return "".to_string()
    }
    // res_vec.sort_by(|a, b| my_scuffed_sorting_algorithm(*a, *b));
    println!("{}", res_vec.len());
    let mut pop_index = {
        if 1 >= res_vec.len() {
            0
        } else {
            res_vec.len() - 1
        }
    };
    while res_vec[pop_index].2 <= 1 {
        res_vec.pop();
        pop_index = {
            if 0 >= res_vec.len() {
                break;
                } else {
                res_vec.len() - 1
            }
        };
    };
    res_vec.sort_by(|a, b| my_scuffed_sorting_algorithm(*a, *b));
    println!("{:?}", res_vec);
    let res_str = res_vec.iter().map(|(s, c, n)| to_mix_substring(*s, *c, *n)).collect::<String>();
    res_str[0..res_str.len() - 1].to_string()
}

fn my_scuffed_sorting_algorithm(a: (char, char, usize), b: (char, char, usize))-> Ordering {
    let cmp = b.2.partial_cmp(&a.2).unwrap();
    match cmp {
        Ordering::Equal => {
            let t = match (a.0, b.0) {
                (_a, _b ) if (_a == '1') & (_b == '1') => Ordering::Equal,
                (_a, _b ) if (_a == '1') & (_b == '2') => Ordering::Less,
                (_a, _b ) if (_a == '1') & (_b == '=') => Ordering::Less,
                (_a, _b ) if (_a == '2') & (_b == '1') => Ordering::Greater,
                (_a, _b ) if (_a == '2') & (_b == '2') => Ordering::Equal,
                (_a, _b ) if (_a == '2') & (_b == '=') => Ordering::Less,
                (_a, _b ) if (_a == '=') & (_b == '1') => Ordering::Greater,
                (_a, _b ) if (_a == '=') & (_b == '2') => Ordering::Greater,
                (_a, _b ) if (_a == '=') & (_b == '=') => Ordering::Equal,
                _ => Ordering::Less,
            };
            if t != Ordering::Equal {
                t
            } else {
                let some = {if (a.0 == '=') & (b.0 != '=') {
                    Ordering::Greater
                } else if (a.0 != '=') & (b.0 == '=') {
                    // println!("A={}, B={}, Res={:?}", a.1, b.1, a.1.partial_cmp(&b.1).unwrap());
                    Ordering::Less
                } else {
                    // a.1.partial_cmp(&b.1).unwrap()
                    // println!("A={}, B={}", a.1, b.1);
                    a.1.partial_cmp(&b.1).unwrap()
                }};
                // println!("A={}, B={}, Res={:?}", a.1, b.1, some);
                some
            }
        }, 
        _ => cmp
    }
}

#[allow(dead_code)]
fn to_mix_substring(found_in_string: char, character: char, count: usize) -> String {
    if count <= 1 {
        "".to_string()
    } else {
        let mut res = String::from("");
        res.push(found_in_string);
        res.push(':');
        res.push_str(&iter::repeat(character).take(count).collect::<String>());
        res.push('/');
        res
    }
}


#[allow(dead_code)]
pub fn run() {
    // let s1 = "my&friend&Paul has heavy ats! &";
    // let s2 = "my friend John has many many friends &";
    let s3 = "Are they here";
    let s4 = "yes, they are here";
    let res = mix(s3, s4);
    println!("{}", res);
}

#[cfg(test)]
mod tests {
    use super::mix;
    
    #[test]
    fn basics_mix() {
        testing("Are they here", "yes, they are here", 
            "2:eeeee/2:yy/=:hh/=:rr");
        testing("looping is fun but dangerous", "less dangerous than coding", 
            "1:ooo/1:uuu/2:sss/=:nnn/1:ii/2:aa/2:dd/2:ee/=:gg");
        testing(" In many languages", " there's a pair of functions", 
            "1:aaa/1:nnn/1:gg/2:ee/2:ff/2:ii/2:oo/2:rr/2:ss/2:tt");
        testing("Lords of the Fallen", "gamekult", "1:ee/1:ll/1:oo");
        testing("codewars", "codewars", "");
        testing("A generation must confront the looming ", "codewarrs", 
            "1:nnnnn/1:ooooo/1:tttt/1:eee/1:gg/1:ii/1:mm/=:rr");
    }
    
    fn testing(s1: &str, s2: &str, exp: &str) -> () {
        assert_eq!(&mix(s1, s2), exp)
    }
}