mod solution {}

// use std::ops::Fn;
// use std::ops::FnMut;
// use std::ops::FnOnce;

#[allow(dead_code)]
fn format_duration(seconds: u64) -> String {
    if seconds == 0 {
        String::from("now")
    } else {
        let times = vec!["year", "day", "hour", "minute", "second"];
        let mut seconds = seconds;
        let mut minutes = seconds / 60;
        seconds = seconds % 60;
        let mut hours =   minutes / 60;
        minutes = minutes % 60;
        let mut days =    hours   / 24;
        hours = hours % 24;
        let years =   days    / 365;
        days = days % 365;
        let values = vec![years, days, hours, minutes, seconds];
        let mut res: Vec<String> = Vec::new();
        for i in 0..times.len()  {
            let val = values[i];
            let mut time = times[i].to_string();
            if val > 0 {
                if val != 1 {
                    time.push_str("s");
                }
                res.push(format!("{} {}", val, time));
            }
        }
        let k = res.len();
        if k < 2 {
            return res.pop().unwrap();
        }
        let mut head = res.pop().unwrap();
        let before_head = res.pop().unwrap();
        head = format!("{} and {}", before_head, head);
        res.push(head);
        if k == 1 {
            res.join("")
        } else {
            res.join(", ")
        }
    }
}


#[allow(dead_code)]
pub fn run() {
    let _a_year_in_seconds = 60*60*24*365;
    let res = format_duration(_a_year_in_seconds + 60*60*24 + 60*60 + 60 + 1);
    println!("{}", res);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(format_duration(1), "1 second");
        assert_eq!(format_duration(62), "1 minute and 2 seconds");
        assert_eq!(format_duration(120), "2 minutes");
        assert_eq!(format_duration(3600), "1 hour");
        assert_eq!(format_duration(3662), "1 hour, 1 minute and 2 seconds");
    }
}
