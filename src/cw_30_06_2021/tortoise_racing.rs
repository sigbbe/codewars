// Two tortoises named A and B must run a race.
// A starts with an average speed of 720 feet per
// hour. Young B knows she runs faster than A, and
// furthermore has not finished her cabbage.
// When she starts, at last, she can see that A has
// a 70 feet lead but B's speed is 850 feet per hour.
// How long will it take B to catch A? More generally:
// given two speeds v1 (A's speed, integer > 0) and v2
// (B's speed, integer > 0) and a lead g (integer > 0)
// how long will it take B to catch A? The result will
// be an array [hour, min, sec] which is the time needed
// in hours, minutes and seconds (round down to the nearest
// second) or a string in some languages. If v1 >= v2 then
// return nil, nothing, null, None or {-1, -1, -1} for C++,
// C, Go, Nim, Pascal,[] for Kotlin or "-1 -1 -1".

#[allow(dead_code)]
pub fn race(v1: i32, v2: i32, g: i32) -> Option<Vec<i32>> {
    match (v1, v2) {
        (a, b) if a < b => {
            let h: f32 = (g as f32) / (b as f32 - a as f32);
            let m: f32 = (h - h.floor()) * 60.0;
            let s: f32 = (m - m.floor()) * 60.0;
            let res: Vec<i32> = vec![h.floor() as i32, m.round() as i32, s.round() as i32];
            Some(res)
        }
        (a, b) if a >= b => None,
        _ => None,
    }
}
