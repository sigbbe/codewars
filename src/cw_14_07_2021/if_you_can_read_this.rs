fn nato(c: &char) -> &str {
    use std::collections::HashMap;
    let mut nato: HashMap<&char, &str> = HashMap::new();
    nato.insert(&'A', "Alfa");
    nato.insert(&'B', "Bravo");
    nato.insert(&'C', "Charlie");
    nato.insert(&'D', "Delta");
    nato.insert(&'E', "Echo");
    nato.insert(&'F', "Foxtrot");
    nato.insert(&'G', "Golf");
    nato.insert(&'H', "Hotel");
    nato.insert(&'I', "India");
    nato.insert(&'J', "Juliett");
    nato.insert(&'K', "Kilo");
    nato.insert(&'L', "Lima");
    nato.insert(&'M', "Mike");
    nato.insert(&'N', "NORMA");
    nato.insert(&'O', "Oscar");
    nato.insert(&'P', "Papa");
    nato.insert(&'Q', "Quebec");
    nato.insert(&'R', "Romeo");
    nato.insert(&'S', "Sierra");
    nato.insert(&'T', "Tango");
    nato.insert(&'U', "Uniform");
    nato.insert(&'V', "Victor");
    nato.insert(&'W', "Whiskey");
    nato.insert(&'X', "X-ray");
    nato.insert(&'Y', "Yankee");
    nato.insert(&'Z', "Zulu");
    nato[c]
}

#[allow(dead_code)]
fn to_nato(words: &str) -> String {
    let capitol = words
        .to_ascii_uppercase()
        .chars()
        .filter(|c| c != &' ')
        .collect::<Vec<char>>();
    let mut res = String::from("");
    let ascii_a = 'A' as u32;
    let ascii_z = 'Z' as u32;
    for i in 0..capitol.len() {
        let c_i = capitol[i];
        let ascii_c_i = c_i as u32;
        let v: String = [c_i].iter().collect();
        let next: &str = {
            if (ascii_c_i >= ascii_a) & (ascii_c_i <= ascii_z) {
                nato(&c_i)
            } else {
                &v
            }
        };
        res.push_str(next);
        if i < capitol.len() - 1 {
            let some_chars: &[char] = &[' ', ',', '?'];
            if !some_chars.contains(&capitol[i + 1]) {
                res.push_str(" ");
            }
        }
    }
    res
}
#[allow(dead_code)]
pub fn run() {
    let input = "If, you can read?";
    println!("{}: {}", input, to_nato(input));
}
