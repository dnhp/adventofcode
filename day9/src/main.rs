use std::fs::File;
use std::io::prelude::*;

fn main() {
	
	let mut f = File::open("input.txt").unwrap();
	let mut contents = String::new();
	f.read_to_string(&mut contents).unwrap();

	let mut garbage = false;
	let mut skip_next = false;
	let mut group_depth: usize = 0;
	let mut score: usize = 0;
	let mut n_groups: usize = 0;
	let mut chars_in_garbage: usize = 0;
	
	for c in contents.chars() {
		if !garbage {
			match c {
				'{' => {
					group_depth += 1;
					n_groups +=1;
				},
				'}' => {
					score += group_depth;
					group_depth -= 1;
				},
				'<' => garbage = true,
				_ => {},
			}
		}
		else {
			if !skip_next {
				match c {
					'!' => skip_next = true,
					'>' => garbage = false,
					_ => chars_in_garbage += 1,
				}
			}
			else {
				skip_next = false;
			}
		}
	}

	println!("n_groups = {:?}", n_groups);
	println!("score = {:?}", score);
	println!("chars_in_garbage = {:?}", chars_in_garbage);
}
