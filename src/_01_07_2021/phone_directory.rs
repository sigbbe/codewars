// 1. std::borrow::{ToOwned,Borrow} are pretty important as if you can grok them you basically understand the ownership model.
// 2. std::convert::From is just extremely nice as you can avoid declaring new() all the time.
// 3. std::str::FromStr just lovely since it makes a high level "just parse that data from a string" interface.
// 4. std::ops::Deref & std::convert::AsRef lets you do magic pretending some types are other types. Allows for trait inheritance, flexiable generic parameters.
// 5. std::iterator::Iterator honorable mention because it lets you do SO MUCH!!

use std::borrow::{Borrow, ToOwned};
use std::cmp::Ordering;
use std::convert::AsRef;
use std::convert::From;
use std::fmt::Display;
use std::iter::Iterator;
use std::ops::Deref;
use std::ops::*;
use std::str::FromStr;

// Format:
// +XX-abc-def-ghij, <name>, <address>

// Examples:
// 1. "/+1-541-754-3010 156 Alphand_St. <J Steeve>\n"
// 2. " 133, Green, Rd. <E Kustur> NY-56423 ;+1-541-914-3010!\n"
// 3. "<Anastasia> +48-421-674-8974 Via Quirinal Roma\n"

// use std::str::pattern::Pattern;
// use std::str::pattern::Searcher;

#[derive(Debug, Copy, Clone)]
struct SomeStruct<T: Iterator>(T);
trait SomeTraits<T> {
    type Input;
    type Output;
    fn new(arg: Self::Input) -> Self::Output;
    fn next(&mut self) -> Self::Output;
}
