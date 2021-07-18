#[allow(dead_code)]
pub fn anagrams(word: &str, words: &[String]) -> Vec<String> {
    let mut vec: Vec<String> = vec![];
    for i in 0..words.len() {
        let word_i: &String = &words[i];
        if are_anagrams(word, word_i) {
            vec.push(String::from(word_i));
        }
    }
    vec
}

fn are_anagrams(a: &str, b: &str) -> bool {
    use std::collections::hash_map::RandomState;
    use std::collections::HashSet;
    use std::iter::FromIterator;
    let a = a.to_ascii_lowercase();
    let b = b.to_ascii_lowercase();
    let chars_a = a.chars().collect::<Vec<char>>();
    let chars_b = b.chars().collect::<Vec<char>>();
    if chars_a.len() != chars_b.len() {
        return false;
    }
    let set_of_chars_a: HashSet<&char, RandomState> = HashSet::from_iter(chars_a.iter());
    let set_of_chars_b: HashSet<&char, RandomState> = HashSet::from_iter(chars_b.iter());
    if !set_of_chars_a.eq(&set_of_chars_b) {
        return false;
    }
    let vec_of_unique_chars: Vec<&char> = set_of_chars_a
        .clone()
        .iter()
        .map(|c| *c)
        .collect::<Vec<&char>>();
    for i in 0..vec_of_unique_chars.len() {
        let unique_char = vec_of_unique_chars[i];
        let number_of_a_i: usize = chars_a.iter().filter(|&n| n == unique_char).count();
        let number_of_b_i: usize = chars_b.iter().filter(|&n| n == unique_char).count();
        if number_of_a_i != number_of_b_i {
            return false;
        }
    }
    true
}

pub mod tests {
    use super::*;

    pub fn sample_tests() {
        do_test("abba", &["aabb", "abcd", "bbaa", "dada"], &["aabb", "bbaa"]);
        do_test(
            "racer",
            &["crazer", "carer", "racar", "caers", "racer"],
            &["carer", "racer"],
        );
    }

    pub fn do_test(word: &str, words: &[&str], exp: &[&str]) {
        let words: Vec<String> = words.iter().map(|w| w.to_string()).collect();
        let expected: Vec<String> = exp.iter().map(|w| w.to_string()).collect();
        let got = anagrams(word, &words);
        assert_eq!(
            got, expected,
            "Failed with word: \"{}\"\nwords: {:#?}",
            word, words
        );
    }
}
