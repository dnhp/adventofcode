use std::fs::File;
use std::io::prelude::*;
use std::cmp;

fn main() {
    let mut f = File::open("input.txt").unwrap();
    let mut contents = String::new();
    f.read_to_string(&mut contents).unwrap();

    let input: Vec<&str> = contents.trim()
        .split(',')
        .collect();

    let mut x_coord = 0i32;
    let mut y_coord = 0i32;
    let mut z_coord = 0i32;

    let mut max_dist_from_origin = 0i32;

    for dir in input {
        match dir {
            "n" => {
                x_coord -= 1;
                y_coord += 1;
            },
            "ne" => {
                y_coord += 1;
                z_coord -= 1;
            },
            "se" => {
                x_coord += 1;
                z_coord -= 1;
            },
            "s" => {
                x_coord += 1;
                y_coord -= 1;
            },
            "sw" => {
                y_coord -= 1;
                z_coord += 1;
            },
            "nw" => {
                x_coord -= 1;
                z_coord += 1;
            },
            _ => panic!(),
        }

        let dist_from_origin = get_n_steps_from_origin(x_coord, y_coord, z_coord);
        if dist_from_origin > max_dist_from_origin {
            max_dist_from_origin = dist_from_origin;
        }
    }

    println!("Distance from origin at end [part1]: {:?}", get_n_steps_from_origin(x_coord, y_coord, z_coord));
    println!("Max distance from origin [part2]: {:?}", max_dist_from_origin);
}

fn get_n_steps_from_origin (
    x_coord: i32,
    y_coord: i32,
    z_coord: i32) -> i32 {
    cmp::max(cmp::max(x_coord.abs(), y_coord.abs()), z_coord.abs())
}