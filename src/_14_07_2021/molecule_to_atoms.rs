#[allow(unused_imports)]
use std::collections;
#[allow(unused_imports)]
use std::collections::hash_map::RandomState;
use std::collections::HashSet;
use std::error::Error;
use std::fmt;
use std::iter::FromIterator;
#[allow(unused_imports)]
use std::ops::Deref;

pub type Atom = (String, usize);
pub type Molecule = Vec<Atom>;

#[derive(Debug)]
pub enum ParseError {
    #[allow(dead_code)]
    InvalidAtom,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "ParseError: {:?}", self)
    }
}

impl Error for ParseError {}

#[allow(dead_code)]
type IndexAndChar = (usize, char);

#[allow(dead_code)]
pub fn parse_molecule(s: &str) -> Result<Molecule, ParseError> {
    #[allow(unused_variables)]
    let all_valid_elements = valid_element_symbols();
    let molecule: Molecule = Vec::new();
    #[allow(unused_variables)]
    let split_at_numbers = s
        .split(char::is_numeric)
        .map(|v| String::from(v))
        .collect::<Vec<String>>();
    let mut split_all_chars = s
        .split("")
        .filter(|v| v != &"")
        .map(|v| String::from(v))
        .collect::<Vec<String>>();
    let mut i = 0;
    while i < split_all_chars.len() - 1 {
        let value: &str = split_all_chars[i].as_ref();
        let next: &str = split_all_chars[i + 1].as_ref();
        if !(is_alphabetic(&next)) || !(is_lower_case(&next)) {
            i += 1;
            continue;
        }
        let mut new = String::from(value);
        new.push_str(&next);
        split_all_chars.remove(i);
        split_all_chars.remove(i);
        split_all_chars.insert(i, new);
        i += 1;
    }
    let elements = Vec::from_iter(HashSet::<&String, MyState>::from_iter(
        split_all_chars.iter().filter(|v| is_alphabetic(v)),
    ));
    for _el in elements {
        // let count = 1;
        // println!("{}", _el);
        // for
        // molecule.push(value: T)
    }
    println!(
        "{:?}",
        vec_index_of_element(&split_all_chars, &"O".to_string())
    );
    for _i in split_all_chars {}
    Ok(molecule)
}

fn vec_index_of_element<T: PartialEq>(vec: &Vec<T>, t: &T) -> usize {
    let i = vec.iter().position(|r| r.eq(t)).unwrap_or_default();
}

use std::default::Default;
use std::hash::BuildHasher;
use std::hash::Hasher;

impl BuildHasher for MyState {
    type Hasher = MyState;
    fn build_hasher(&self) -> Self::Hasher {
        MyState { k0: 1, k1: 1 }
    }
}

impl Hasher for MyState {
    fn finish(&self) -> u64 {
        1
    }
    fn write(&mut self, _bytes: &[u8]) {
        // println!("{:?}", &bytes);
    }
}

impl Default for MyState {
    fn default() -> Self {
        Self { k0: 0, k1: 0 }
    }
}

struct MyState {
    #[allow(dead_code)]
    k0: u64,
    #[allow(dead_code)]
    k1: u64,
}

#[allow(dead_code)]
fn is_numeric(s: &str) -> bool {
    s.chars().all(char::is_numeric)
}

#[allow(dead_code)]
fn is_alphabetic(s: &str) -> bool {
    s.chars().all(|i| i.is_alphabetic())
}

#[allow(dead_code)]
fn is_lower_case(s: &str) -> bool {
    s == s.to_ascii_lowercase()
}

#[allow(dead_code)]
fn valid_element_symbols() -> Vec<String> {
    let file = "/home/sigbbe/hackerman/technologies/Rust/codewars/src/_14_07_2021/periodic_table_of_elements.csv";
    let column = "Symbol";
    let symbols = match get_column_as_vec(file, column) {
        Ok(s) => s,
        Err(_) => process::exit(1),
    };
    symbols
}

use std::process;
#[allow(dead_code)]
fn get_column_as_vec(file: &str, column: &str) -> Result<Vec<String>, Box<dyn Error>> {
    // Build the CSV reader and iterate over each record.
    let file = csv::Reader::from_path(file);
    let mut rdr = match file {
        Ok(r) => r,
        Err(e) => {
            println!("error running example: {}", e);
            process::exit(1);
        }
    };
    let column_index = match rdr.headers() {
        Ok(headers) => {
            // pub fn get(&self, i: usize) -> Option<&str>
            let mut i: usize = 0;
            while headers.get(i) != None || i > 100 {
                let header = headers.get(i).unwrap();
                if header == column {
                    break;
                } else {
                    i += 1;
                }
            }
            i
        }
        Err(_) => {
            return Err(Box::new(ParseError::InvalidAtom));
        }
    };
    let mut res = Vec::new();
    for result in rdr.records() {
        let record = result?;
        // The iterator yields Result<StringRecord, Error>, so we check the
        // error here.
        // println!("{:?}", result);
        res.push(String::from(record[column_index].trim()))
    }
    Ok(res)
}

#[allow(unused_imports)]
use meval::eval_str;

pub fn run() {
    // water // Ok([("H", 2), ("O", 1)])
    // println!("{:?}", parse_molecule("H2O"));

    // magnesium hydroxide // Ok([("Mg", 1), ("O", 2), ("H", 2)]
    // println!("{:?}", parse_molecule("Mg(OH)2"));

    // Fremy's salt // Ok([("K", 4), ("O", 14),("N", 2),("S", 4)])
    #[allow(unused_must_use)]
    let _t = parse_molecule("K4[ON(SO3)2]2");
    println!("{:?}", [("K", 4), ("O", 14), ("N", 2), ("S", 4)]);
    // ParseError
    // println!("{:?}", parse_molecule("pie"));
}
