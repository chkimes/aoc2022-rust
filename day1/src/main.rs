use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();

    let mut maxes: [u32; 3] = [0; 3];
    let mut running_total = 0;
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        let line = line.trim_end().to_string();
        if line == "" {
            if running_total > maxes[0] {
                maxes[0] = running_total;
                maxes.sort();
            }

            running_total = 0;
            continue;
        }

        let i = line.parse::<u32>().unwrap();
        running_total += i;
    }

    if running_total > maxes[0] {
        maxes[0] = running_total;
        maxes.sort();
    }

    for max in maxes.iter() {
        println!("{max}");
    }

    let sum: u32 = maxes.iter().sum();
    println!("");
    println!("Sum: {sum}");
}
