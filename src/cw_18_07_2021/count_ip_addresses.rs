
#[allow(dead_code)]
fn ips_between(start: &str, end: &str) -> u32 {
    let pow = &[24, 16, 8, 0];
    let vec_start = start.split(".").map(|v| v.parse::<u32>().unwrap()).collect::<Vec<u32>>();
    let vec_end = end.split(".").map(|v| v.parse::<u32>().unwrap()).collect::<Vec<u32>>();
    let ip_to_u32 = |ip: &Vec<u32>| -> u32 {
        let mut tmp = vec![];
        for i in 0..ip.len() {
            tmp.push(u32::pow(2, pow[i])*ip[i]);
        }
        tmp.iter().sum()
    };
    let start_val = ip_to_u32(&vec_start);
    let end_val = ip_to_u32(&vec_end);
    end_val - start_val
}
// println!("{}: {}", start, start_val);
// println!("{}: {}", end, end_val);



#[allow(dead_code)]
pub fn run() {
    println!("{}", u32::MAX);
}



#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(ips_between("10.0.0.0", "10.0.0.50"), 50);
        assert_eq!(ips_between("20.0.0.10", "20.0.1.0"), 246);
    }
}