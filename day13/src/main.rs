extern crate num;

use std::fs::File;
use std::io::{BufRead,BufReader};
use num::integer::lcm;

#[derive(Debug,Clone,Copy)]
struct Scanner {
    depth_id: usize,
    range: usize,
    current_pos: usize,
    move_dir: char,
}

impl Scanner {
    fn new(depth_id: usize, range: usize) -> Scanner {
        Scanner {
            depth_id,
            range,
            current_pos: 0,
            move_dir: 'd',
        }    
    }
}

struct ScannerArray {
    depth: usize,
    scanner_vec: Vec<Scanner>,
    positions_with_scanners: Vec<bool>,
    orig_scanner_vec: Vec<Scanner>
}

impl ScannerArray {
    fn new (depth: usize, scanner_vec: Vec<Scanner>) -> ScannerArray {
        
        let pws: Vec<bool> = (0..depth)
                .into_iter()
                .map(|id| scanner_vec.iter()
                            .map(|sc| sc.depth_id)
                            .collect::<Vec<usize>>()
                            .contains(&id))
                .collect();

        let osv = scanner_vec.clone();

        ScannerArray {
            depth,
            scanner_vec,
            positions_with_scanners: pws,
            orig_scanner_vec: osv,
        }
    }

    fn reset(&mut self) {
        self.scanner_vec = self.orig_scanner_vec.clone();
    }

    fn increment_positions(&mut self) {
        for scanner in self.scanner_vec.iter_mut() {
            // Check if scanner is at range limits - if so,
            // reverse direction of motion
            match scanner.current_pos {
                0 => scanner.move_dir = 'd',
                _ if scanner.current_pos == scanner.range-1 => scanner.move_dir = 'u',
                _ => {},
            }
            // Check direction of motion of scanner and update
            // position
            match scanner.move_dir {
                'd' => scanner.current_pos += 1,
                'u' => scanner.current_pos -= 1,
                _ => panic!("unrecognised direction"),
            }
        }
    }

    fn get_all_scanner_pos (&self) -> Vec<usize> {
        self.scanner_vec.iter()
            .map(|sc| sc.current_pos)
            .collect()
    }

    fn get_all_scanner_range (&self) -> Vec<usize> {
        self.scanner_vec.iter()
            .map(|sc| sc.range)
            .collect()
    }

    fn get_single_scanner_pos (&self, depth: usize) -> Option<usize> {

        if self.positions_with_scanners[depth] {
            let pos: usize = self.scanner_vec.iter()
                                .find(|sc| sc.depth_id == depth)
                                .unwrap()
                                .current_pos;
            Some(pos)
        }
        else {
            None
        }
    }

    fn get_single_scanner_range (&self, depth: usize) -> Option<usize> {

        if self.positions_with_scanners[depth] {
            let range: usize = self.scanner_vec.iter()
                                .find(|sc| sc.depth_id == depth)
                                .unwrap()
                                .range;
            Some(range)
        }
        else {
            None
        }
    }
}

fn main() {

    let input_file = File::open("input.txt").unwrap();
    let buf = BufReader::new(input_file);


    let scan_dat: Vec<Vec<usize>> = buf.lines()
        .map(|l| l.unwrap()
            .split(": ")
            .map(|d| d.parse()
                .unwrap())
                .collect())
        .collect();

    let mut scanner_vec: Vec<Scanner> = Vec::new();

    for scan_ln in scan_dat {
        scanner_vec.push(Scanner::new(scan_ln[0], scan_ln[1]));
    }

    let depth: usize = scanner_vec.iter().map(|sc| sc.depth_id).max().unwrap() + 1;
    let mut scanner_array = ScannerArray::new(depth, scanner_vec);

    let delay_part1 = 0;
    let (severity,_) = release_packet(&mut scanner_array, delay_part1);
    println!("Final severity [part1]: {:?}", severity);

    let all_ranges: Vec<usize> = scanner_array.get_all_scanner_range()
                        .iter()
                        .map(|r| 2*r)
                        .collect();

    let lcm: usize = all_ranges.iter()
                .fold(all_ranges[0], |acc, &val| lcm(acc,val));
    println!("LCM: {:?}", lcm);
    
    let mut delay = 0;
    let mut captured = true;
    let mut res: (usize, bool);
    while captured {
        delay += 1;
        if delay > lcm {
            panic!("Too many cycles");
        }
        scanner_array.reset();
        res = release_packet(&mut scanner_array, delay);
        captured = res.1;

    }
    println!("Delay to avoid capture [part2]: {:?}", delay);
    
}

fn release_packet(scanner_array: &mut ScannerArray, delay: usize) -> (usize, bool) {

    let mut packet_pos = 0;
    let mut severity = 0;
    let mut captured = false;

    for time in 0..scanner_array.depth+delay {
        if time >= delay {
            if let Some(pos) = scanner_array.get_single_scanner_pos(packet_pos) {
                if pos == 0 {
                    // Captured
                    //severity += packet_pos*scanner_array.get_single_scanner_range(packet_pos).unwrap();
                    captured = true;
                    return (0,captured);
                }
            }
        }
        
        scanner_array.increment_positions();
        if time >= delay {
            packet_pos += 1;
        }
    }
    //(severity,captured)
    (0,captured)
}