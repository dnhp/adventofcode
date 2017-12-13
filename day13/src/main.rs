use std::fs::File;
use std::io::{BufRead,BufReader};
use std::collections::HashMap;

fn main() {

    let input_file = File::open("input.txt").unwrap();
    let buf = BufReader::new(input_file);

    let scan_dat: Vec<Vec<usize>> = buf.lines()
        .map(|l| l.unwrap()
            .split(": ")
            .map(|d| d.parse().unwrap())
            .collect())
        .collect();

    let mut scanners = HashMap::new();
    for scan_ln in scan_dat {
        scanners.insert(scan_ln[0], scan_ln[1]);
    }

    let severity_delay_0 = scanners.iter()
        .filter(|&(&depth, &range)| (depth % (2*(range-1))) == 0)
        .fold(0, |acc, (&depth, &range)| acc + depth*range);

    println!("Severity at delay 0 [part1]: {:?}", severity_delay_0);

    let mut delay=0;
    while scanners.iter().any(|(&depth,&range)| (depth+delay) % (2*(range-1)) == 0) {
        delay += 1;
    }

    println!("Delay to avoid capture [part2] {:?}", delay);
}