#[allow(dead_code)]
pub fn range_extraction(a: &[i32]) -> String {
    let mut res: Vec<String> = Vec::new();
    let mut i = 0;
    while i < a.len() {
        let val = a[i];
        let mut range_len = 1;
        let mut j = { if i < a.len() - 1 { i + 1} else { a.len() - 1 } };
        println!("{},{}", j, j-1);
        'l: while a[j] == a[j - 1] + 1 {
            range_len += 1;
            j += 1;
            if j >= a.len() {
                break 'l;
            }
        }
        if range_len >= 3 {
            i = i + range_len - 1;
            let mut rng = String::from(val.to_string());   
            rng.push_str("-");
            rng.push_str(&a[i].to_string());
            res.push(rng);
        } else {
            res.push(val.to_string());
        }
        i += 1;
    }
    res.join(",")
}

#[allow(dead_code)]
pub fn run() {
    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!("-6,-3-1,3-5,7-11,14,15,17-20", range_extraction(&[-6,-3,-2,-1,0,1,3,4,5,7,8,9,10,11,14,15,17,18,19,20]));	
        assert_eq!("-3--1,2,10,15,16,18-20", range_extraction(&[-3,-2,-1,2,10,15,16,18,19,20]));
    }
}
