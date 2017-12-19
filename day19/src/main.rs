use std::fs::File;
use std::io::{BufRead,BufReader};

fn main() {

    let input_file = File::open("input.txt").unwrap();
    let buf = BufReader::new(input_file);

    let map: Vec<Vec<char>> = buf.lines()
        .map(|l| l.unwrap().chars().collect())
        .collect();

    let puzzle_width = map[0].len() as isize;
    let puzzle_height = map.len() as isize;

    let start_horz_coord = map[0].iter().position(|&c| c == '|').unwrap();
    let mut current_pos: Vec<isize> = vec![0,start_horz_coord as isize];

    let mut inc: isize = 1;
    let mut ver = true;
    let mut next_char = map[current_pos[0] as usize][current_pos[1] as usize];

    let mut n_steps = 1;
    print!("Letter sequence [part1]: ");
    while next_char != ' ' {
        while next_char != '+' {
            if ver {
                current_pos[0] = current_pos[0] + inc;
                next_char = map[current_pos[0] as usize][current_pos[1] as usize];
            }
            else {
                current_pos[1] = current_pos[1] + inc;
                next_char = map[current_pos[0] as usize][current_pos[1] as usize];
            }

            n_steps += 1;
            if next_char.is_alphabetic() {
                print!("{}", next_char);
                if next_char == 'P' {
                    println!("\nNumber of steps [part2]: {:?}", n_steps);
                    return;
                }
            }
        }

        ver = !ver; // change direction from horizontal to vertical

        // Figure out which way to go next
        if ver {
            // Check up and down
            if current_pos[0] != puzzle_height-1 && map[current_pos[0] as usize + 1][current_pos[1] as usize] != ' ' {
                // Go down
                inc = 1;
            }
            else if current_pos[0] != 0 && map[current_pos[0] as usize - 1][current_pos[1] as usize] != ' ' {
                // Go up
                inc = -1;
            }
            next_char = map[(current_pos[0]+ inc) as usize][current_pos[1] as usize];

        }
        else {
            // Check left and right
            if current_pos[1] != puzzle_width-1 && map[current_pos[0] as usize][current_pos[1] as usize + 1] != ' ' {
                // Go right
                inc = 1;
            }
            else if current_pos[1] != 0 && map[current_pos[0] as usize][current_pos[1] as usize - 1] != ' ' {
                // Go left
                inc = -1;
            }
            next_char = map[current_pos[0] as usize][(current_pos[1] + inc) as usize];
        }
    }
}
