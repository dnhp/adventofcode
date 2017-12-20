extern crate regex;
extern crate nalgebra;
extern crate integer_sqrt;

use std::fs::File;
use std::io::{BufRead,BufReader};
use regex::Regex;
use nalgebra::{Determinant,Matrix3};

fn main() {

    let input_file = File::open("input.txt").unwrap();
    let buf = BufReader::new(input_file);
;
    let re = Regex::new(r"p=<(-?\d+),(-?\d+),(-?\d+)>, v=<(-?\d+),(-?\d+),(-?\d+)>, a=<(-?\d+),(-?\d+),(-?\d+)>").unwrap();

    let mut pos_vec: Vec<(isize,isize,isize)> = Vec::new();
    let mut vel_vec: Vec<(isize,isize,isize)> = Vec::new();
    let mut acc_vec: Vec<(isize,isize,isize)> = Vec::new();

    for line in buf.lines().map(|l| l.unwrap()) {

        let caps = re.captures(&line).unwrap();

        let pos: (isize,isize,isize) = (caps.get(1).unwrap().as_str().parse().unwrap(), 
                                        caps.get(2).unwrap().as_str().parse().unwrap(),
                                        caps.get(3).unwrap().as_str().parse().unwrap());
        pos_vec.push(pos);

        let vel: (isize,isize,isize) = (caps.get(4).unwrap().as_str().parse().unwrap(), 
                                        caps.get(5).unwrap().as_str().parse().unwrap(),
                                        caps.get(6).unwrap().as_str().parse().unwrap());
        vel_vec.push(vel);

        let acc: (isize,isize,isize) = (caps.get(7).unwrap().as_str().parse().unwrap(), 
                                        caps.get(8).unwrap().as_str().parse().unwrap(),
                                        caps.get(9).unwrap().as_str().parse().unwrap());
        acc_vec.push(acc);
    }


    let mut collision_t_matrix: Vec<Vec<isize>> = vec![vec![isize::max_value(); pos_vec.len()]; pos_vec.len()];

    for p1 in 0..pos_vec.len()-1 {

        for p2 in p1+1..pos_vec.len() {

            let m2 = Matrix3::new(
                              2*(pos_vec[p1].0-pos_vec[p2].0),
                              2*(vel_vec[p1].0-vel_vec[p2].0) + acc_vec[p1].0-acc_vec[p2].0,
                              acc_vec[p1].0-acc_vec[p2].0,

                              2*(pos_vec[p1].1-pos_vec[p2].1),
                              2*(vel_vec[p1].1-vel_vec[p2].1) + acc_vec[p1].1-acc_vec[p2].1,
                              acc_vec[p1].1-acc_vec[p2].1,

                              2*(pos_vec[p1].2-pos_vec[p2].2),
                              2*(vel_vec[p1].2-vel_vec[p2].2) + acc_vec[p1].2-acc_vec[p2].2,
                              acc_vec[p1].2-acc_vec[p2].2);

            if m2.determinant() == 0 {

                // Will collide - determine time of collision
                let mut pos1 = pos_vec[p1];
                let mut vel1 = vel_vec[p1];
                let mut pos2 = pos_vec[p2];
                let mut vel2 = vel_vec[p2];
                let mut t = 0;
                while pos1 != pos2 {
                    vel1.0 = vel1.0 + acc_vec[p1].0;
                    vel1.1 = vel1.1 + acc_vec[p1].1;
                    vel1.2 = vel1.2 + acc_vec[p1].2;
                    pos1.0 = pos1.0 + vel1.0;
                    pos1.1 = pos1.1 + vel1.1;
                    pos1.2 = pos1.2 + vel1.2;

                    vel2.0 = vel2.0 + acc_vec[p2].0;
                    vel2.1 = vel2.1 + acc_vec[p2].1;
                    vel2.2 = vel2.2 + acc_vec[p2].2;
                    pos2.0 = pos2.0 + vel2.0;
                    pos2.1 = pos2.1 + vel2.1;
                    pos2.2 = pos2.2 + vel2.2;

                    t += 1;
                    if t > 1000 {
                      t = isize::max_value();
                      break;
                    }
                }
                let t_c0 = t;

                if t_c0 < collision_t_matrix[p1][p2] {
                  collision_t_matrix[p1][p2] = t_c0;
                }
            }                    
        }
    }

    let mut col_m_cl = collision_t_matrix.clone();

    for p1 in 0..pos_vec.len()-1 {

        for p2 in p1+1..pos_vec.len() {
          let cp_val = col_m_cl[p1][p2];
          col_m_cl[p2][p1] = cp_val;
        }
    }

    for p1 in 0..pos_vec.len()-1 {
        let min_val = col_m_cl[p1].iter().min().unwrap().clone();
        for p2 in p1+1..pos_vec.len() {
          if col_m_cl[p1][p2] != min_val {
            col_m_cl[p1][p2] = isize::max_value();
            col_m_cl[p2][p1] = isize::max_value();
          }
        }
    }

    let n_p = col_m_cl.iter()
        .filter(|&ln| *ln == vec![isize::max_value(); pos_vec.len()])
        .count();
    println!("Number of particles remaining: {:?}", n_p);
}
