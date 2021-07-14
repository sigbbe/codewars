// Write a function that takes in a string of one or more words,
// and returns the same string, but with all five or more letter
// words reversed (like the name of this kata).
// - Strings passed in will consist of only letters and spaces.
// - Spaces will be included only when more than one word is present.

use std::iter::FromIterator;

fn spin_words(words: &str) -> String {
    let mut res = String::from("");
    let words_vec = words.split_ascii_whitespace();
    for w in words_vec {
        let w_chars = w.chars().collect::<Vec<char>>();
        if w_chars.len() < 5 {
            res.push_str(w);
        } else {
            let w_rev =
                &String::from_iter(w_chars.clone().into_iter().rev().collect::<Vec<char>>());
            res.push_str(w_rev);
        }
        res.push_str(" ");
    }
    String::from(res.trim())
}

pub fn run() {
    let _test_1 = spin_words("Hey fellow warriors");
}
