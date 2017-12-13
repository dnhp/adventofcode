extern crate num;

use std::fs::File;
use std::io::{BufRead,BufReader};
use std::collections::HashMap;

fn main() {

    let input_file = File::open("input.txt").unwrap();
    let buf = BufReader::new(input_file);

    let mut scanners = HashMap::new();

    let scan_dat: Vec<Vec<usize>> = buf.lines()
        .map(|l| l.unwrap()
            .split(": ")
            .map(|d| d.parse().unwrap())
            .collect())
        .collect();

    for scan_ln in scan_dat {
        scanners.insert(scan_ln[0], scan_ln[1]);
    }

    println!("Severity at delay 0 [part1]: {:?}", severity_delay_0(&scanners));

    let mut delay_esc=0;
    for delay in 0..usize::max_value() {
        if released_packet_escaped(&scanners, delay) {
            delay_esc = delay;
            break;
        }
    }

    println!("Delay to avoid capture [part2] {:?}", delay_esc);
}

fn released_packet_escaped(scanners: &HashMap<usize,usize>, delay: usize) -> bool {

    scanners.iter()
        .filter(|&(&depth, &range)| ((depth + delay) % (2*(range-1))) == 0)
        .count() == 0
}

fn severity_delay_0(scanners: &HashMap<usize,usize>) -> usize {
    scanners.iter()
        .filter(|&(&depth, &range)| (depth % (2*(range-1))) == 0)
        .fold(0, |acc, (&depth, &range)| acc + depth*range)
}