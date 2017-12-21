extern crate regex;

use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead,BufReader};
use regex::Regex;

fn rot90_ac (input: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    if input.len() == 2 {
        let mut rot_clone = input.clone();
        rot_clone[1][0] = input[0][0];
        rot_clone[1][1] = input[1][0];
        rot_clone[0][1] = input[1][1];
        rot_clone[0][0] = input[0][1];
        rot_clone
    }
    else if input.len() == 3 {
        let mut rot_clone = input.clone();
        rot_clone[0][0] = input[0][2];
        rot_clone[1][0] = input[0][1];
        rot_clone[2][0] = input[0][0];
        rot_clone[0][1] = input[1][2];
        rot_clone[2][1] = input[1][0];
        rot_clone[0][2] = input[2][2];
        rot_clone[1][2] = input[2][1];
        rot_clone[2][2] = input[2][0];
        rot_clone
    }
    else {
        panic!("Input block too large.");
    }
}

fn reflect (input: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    if input.len() == 2 {
        let mut rot_clone = input.clone();
        rot_clone[0][0] = input[0][1];
        rot_clone[0][1] = input[0][0];
        rot_clone[1][0] = input[1][1];
        rot_clone[1][1] = input[1][0];
        rot_clone
    }
    else if input.len() == 3 {
        let mut rot_clone = input.clone();
        rot_clone[0][0] = input[0][2];
        rot_clone[0][2] = input[0][0];
        rot_clone[1][0] = input[1][2];
        rot_clone[1][2] = input[1][0];
        rot_clone[2][0] = input[2][2];
        rot_clone[2][2] = input[2][0];
        rot_clone
    }
    else {
        panic!("Input block too large.");
    }
}

fn main() {

    let start_pattern = vec![vec!['.', '#', '.'],
                            vec!['.', '.', '#'],
                            vec!['#', '#', '#']];

    let input_file = File::open("test.txt").unwrap();
    let buf = BufReader::new(input_file);

    let re = Regex::new(r"([./#]+) => ([./#]+)").unwrap();

    let mut pattern_store = HashMap::new();

    for line in buf.lines().map(|l| l.unwrap()) {

        for caps in re.captures_iter(&line) {

            println!("pattern: {:?}\trule: {:?}", &caps[1], &caps[2]);
            let pattern: Vec<Vec<char>> = caps[1].split('/')
                .map(|ln| ln.chars().collect())
                .collect();

            let mutation: Vec<Vec<char>> = caps[2].split('/')
                .map(|ln| ln.chars().collect())
                .collect();

            pattern_store.insert(pattern.clone(), mutation.clone());
            pattern_store.insert(rot90_ac(&pattern), mutation.clone());
            pattern_store.insert(rot90_ac(&rot90_ac(&pattern)), mutation.clone());
            pattern_store.insert(rot90_ac(&rot90_ac(&rot90_ac(&pattern))), mutation.clone());

            pattern_store.insert(reflect(&pattern), mutation.clone());
            pattern_store.insert(rot90_ac(&reflect(&pattern)), mutation.clone());
            pattern_store.insert(rot90_ac(&rot90_ac(&reflect(&pattern))), mutation.clone());
            pattern_store.insert(rot90_ac(&rot90_ac(&rot90_ac(&reflect(&pattern)))), mutation.clone());

            /*if pattern.len() == 2 {
                println!("pattern:\n{:?}\n{:?}", pattern[0],pattern[1]);
                println!("pattern_rot:\n{:?}\n{:?}", pattern_rot[0],pattern_rot[1]);
            }
            else {
                println!("pattern:\n{:?}\n{:?}\n{:?}", pattern[0],pattern[1],pattern[2]);
                println!("pattern_rot:\n{:?}\n{:?}\n{:?}", pattern_rot[0],pattern_rot[1],pattern_rot[2]);
            }*/
        }
    }
    let mut previous_patterns: Vec<Vec<Vec<char>>> = Vec::new();
    previous_patterns.push(start_pattern);
    // let mut previous_patterns = start_pattern.clone();

    for iteration in 0..5 {

            let (size,stride): (usize,usize) = if iteration % 2 == 0 { // even iteration
                (3*(2usize.pow(iteration/2)), 3)
            }
            else { // Odd iteration
                (4*(2usize.pow((iteration-1)/2)), 2)
            };
            println!("size: {:?}", size);
            vec![vec!['.'; size]; size];
        
        /*if pattern_store.contains_key(&previous_patterns[iteration as usize]) {
            // let new_pattern = pattern_store.get(&start_pattern).unwrap();
            // println!("new pattern:\n{:?}\n{:?}\n{:?}\n{:?}", new_pattern[0],new_pattern[1],new_pattern[2],new_pattern[3]);
        }*/
    }
}
