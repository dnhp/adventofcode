extern crate regex;
extern crate nalgebra;

use std::fs::File;
use std::io::{BufRead,BufReader};
use regex::Regex;
use nalgebra::{DMatrix3,Determinant,Matrix3};

fn main() {

    let input_file = File::open("test.txt").unwrap();
    let buf = BufReader::new(input_file);

    // let re_pos = Regex::new(r"p=<(-?\d+),(-?\d+),(-?\d+)>").unwrap();
    // let re_vel = Regex::new(r"v=<(-?\d+),(-?\d+),(-?\d+)>").unwrap();
    //let re_acc = Regex::new(r"a=<(-?\d+),(-?\d+),(-?\d+)>").unwrap();
    let re = Regex::new(r"p=<(-?\d+),(-?\d+),(-?\d+)>, v=<(-?\d+),(-?\d+),(-?\d+)>, a=<(-?\d+),(-?\d+),(-?\d+)>").unwrap();

    let mut max_manhat_dist: (usize, isize) = (0, 0);
    let mut min_manhat_dist: (usize,isize) = (0,999999);

    let mut pos_vec: Vec<(isize,isize,isize)> = Vec::new();
    let mut vel_vec: Vec<(isize,isize,isize)> = Vec::new();
    let mut acc_vec: Vec<(isize,isize,isize)> = Vec::new();

    for (p_id,line) in buf.lines().map(|l| l.unwrap()).enumerate() {

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
        //println!("caps: {:?}", acc);

/*        let mat: DMatrix3<isize> = DMatrix3::new_zeros(3,3);
        println!("mat: {:?}",mat );

        let dist = acc.0.abs() + acc.1.abs() + acc.2.abs();
        if dist > max_manhat_dist.1 {
            max_manhat_dist.0 = p_id;
            max_manhat_dist.1 = dist;
        }
        else if dist < min_manhat_dist.1 {
            min_manhat_dist.0 = p_id;
            min_manhat_dist.1 = dist;
        }*/

    }
    println!("pos vec: {:?}", pos_vec);

    let mut remaining_pids: Vec<usize> = (0..1000).collect();

    let mut mat: DMatrix3<isize> = DMatrix3::new_zeros(3,3);

    println!("Biggest accel: {:?}", min_manhat_dist);

    for p1 in 0..1 {
        for p2 in p1+1..2 {
            /*let delta_r = vec![pos_vec[p1].0-pos_vec[p2].0, 
                               pos_vec[p1].1-pos_vec[p2].1,
                               pos_vec[p1].2-pos_vec[p2].2];

            let delta_v = vec![vel_vec[p1].0-vel_vec[p2].0, 
                               vel_vec[p1].1-vel_vec[p2].1,
                               vel_vec[p1].2-vel_vec[p2].2];

            let delta_a = vec![acc_vec[p1].0-acc_vec[p2].0, 
                               acc_vec[p1].1-acc_vec[p2].1,
                               acc_vec[p1].2-acc_vec[p2].2]; */

            let delta_vec = vec![pos_vec[p1].0-pos_vec[p2].0, 
                               pos_vec[p1].1-pos_vec[p2].1,
                               pos_vec[p1].2-pos_vec[p2].2,

                              vel_vec[p1].0-vel_vec[p2].0, 
                              vel_vec[p1].1-vel_vec[p2].1,
                              vel_vec[p1].2-vel_vec[p2].2,

                              acc_vec[p1].0-acc_vec[p2].0, 
                              acc_vec[p1].1-acc_vec[p2].1,
                              acc_vec[p1].2-acc_vec[p2].2];    
                              println!("vec: {:?}",delta_vec ); 

            let m2 = Matrix3::new(pos_vec[p1].0-pos_vec[p2].0, 
                               pos_vec[p1].1-pos_vec[p2].1,
                               pos_vec[p1].2-pos_vec[p2].2,

                              vel_vec[p1].0-vel_vec[p2].0, 
                              vel_vec[p1].1-vel_vec[p2].1,
                              vel_vec[p1].2-vel_vec[p2].2,

                              acc_vec[p1].0-acc_vec[p2].0, 
                              acc_vec[p1].1-acc_vec[p2].1,
                              acc_vec[p1].2-acc_vec[p2].2);

            let mat = DMatrix3::from_column_vector(3,3,&delta_vec);
            println!("mat:\n{:?}", mat);
            println!("m2: {:?}", m2);

            // if mat.determinant() == 0 {
            //     println!("collide {:?} and {:?}", p1, p2);
            // }                      
        }
    }
}
