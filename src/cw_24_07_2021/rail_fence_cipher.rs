mod solution {}

#[allow(dead_code)]
fn encode_rail_fence_cipher(text: &str, num_rails: usize) -> String {
    let chars: Vec<char> = text.chars().collect();
    let l = chars.len();
    let mut rails: Vec<Vec<Option<char>>> = vec![vec![Option::None; l]; num_rails];
    for i in 0..l {
        let current_rail = sin(i, (num_rails - 1) as isize, 0) as usize;
        rails[current_rail][i] = Some(chars[i]);
    }
    rails.iter().map(|rail| rail.iter().filter(|o| *o != &None).map(|s| s.unwrap()).collect::<String>()).collect::<Vec<String>>().join("")
}

#[allow(dead_code)]
fn decode_rail_fence_cipher(text: &str, num_rails: usize) -> String {
    let chars: Vec<char> = text.chars().collect();
    let l = chars.len();
    let mut rails: Vec<Vec<Option<char>>> = vec![vec![Option::None; l]; num_rails];
    let mut res = String::new();
    for i in 0..l {
        rails[sin(i, (num_rails - 1) as isize, 0) as usize][i] = Some('*');
    }
    let mut index = 0;
    for i in 0..num_rails {
        for j in 0..l {
            if let Some(_) = rails[i][j] {
                if index < l {
                    rails[i][j] = Some(chars[index]);
                    index += 1;
                }
            }
        }
    }
    for i in 0..l {
        res.push(rails[sin(i, (num_rails - 1) as isize, 0) as usize][i].unwrap());
    }
    res
}

fn sin(x: usize, upper_limit: isize, lower_limit: isize) -> isize {
    use std::f64::consts::PI;
    let diff = (upper_limit - lower_limit) as f64;
    let mut x = x as f64;
    // To make the sine wave start at zero, 0 => y = 0
    let three_pi_div_two: f64 = PI * 3. / 2.;
    x *= PI / diff;
    x += three_pi_div_two;
    x = x.sin();
    x *= diff / 2.;
    x += diff / 2.;
    x.round() as isize
}

#[allow(dead_code)]
pub fn run() {
    let string_to_be_encoded = "Hello, World!";
    let string_to_be_decoded = "Hoo!el,Wrdl l";
    let number_of_rails = 4;
    let res_encoded = encode_rail_fence_cipher(string_to_be_encoded, number_of_rails);
    let res_decoded = decode_rail_fence_cipher(string_to_be_decoded, number_of_rails);
    println!("{} => {}", string_to_be_encoded, res_encoded);
    println!("{} => {}", string_to_be_decoded, res_decoded);
    // // let i = 1;
    // for i in 0..15 {
    //     let x = sin(i, number_of_rails as isize, 0) as usize;
    //     println!("{} => {}", i, x);
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_tests() {
        assert_eq!(encode_rail_fence_cipher("WEAREDISCOVEREDFLEEATONCE", 3), "WECRLTEERDSOEEFEAOCAIVDEN");
        assert_eq!(decode_rail_fence_cipher("WECRLTEERDSOEEFEAOCAIVDEN", 3), "WEAREDISCOVEREDFLEEATONCE");
        assert_eq!(encode_rail_fence_cipher("Hello, World!", 3), "Hoo!el,Wrdl l");
        assert_eq!(decode_rail_fence_cipher("Hoo!el,Wrdl l", 3), "Hello, World!");
    }
}
