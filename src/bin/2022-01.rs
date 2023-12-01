use std::io::stdin;

fn main() {
    let mut curr_cal = 0;
    let mut first = 0;
    let mut second = 0;
    let mut third = 0;

    for line in stdin().lines() {
        let content = line.unwrap();
        if content.is_empty() {
            if curr_cal > first {
                third = second;
                second = first;
                first = curr_cal;
            } else if curr_cal > second {
                third = second;
                second = curr_cal;
            } else if curr_cal > third {
                third = curr_cal;
            }
            curr_cal = 0;
        } else {
            let calories: i32 = content.parse().unwrap();
            curr_cal += calories;
        }
    }

    // --- Part One ---
    println!("{first}");

    // --- Part Two ---
    let total = first + second + third;
    println!("{total}");
}
