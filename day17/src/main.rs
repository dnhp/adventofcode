use std::collections::HashMap;

fn main() {

    let step_size = 324;
    let num_insertions_p1 = 2018;
    let val_to_find_p1 = 2017;
    
    let solution_p1 = find_solution(step_size, num_insertions_p1, &val_to_find_p1);
    println!("Value after 2017 [part1]: {:?}", solution_p1);

    let num_insertions_p2 = 50_000_001;
    let val_to_find_p2 = 0;

    let mut pos = 0;
    let mut ans = 0;
    for i_op in 1..num_insertions_p2 {

       pos = (pos + step_size) % i_op;
        if pos == val_to_find_p2 {
            ans = i_op;
        }
        pos += 1;
    }
    println!("Value after 0 [part2]: {:?}", ans);
}


fn find_solution (step_size: usize, num_insertions: usize, val_to_find: &usize) -> usize {

    let mut map = HashMap::with_capacity(num_insertions);
    let mut pos: usize = 0;

    map.insert(0,0); // value, position
    
    for i_op in 1..num_insertions {
        if i_op == 1 {
            map.insert(i_op, 1);
            pos += 1;
        }
        else {
            pos = (pos + step_size + 1) % i_op;

            for pos_in_map in map.values_mut().filter(|pos_in| **pos_in >= pos) {
                *pos_in_map += 1;
            }
            
            map.insert(i_op, pos);
        }
    }

    let pos_val_to_find = map.get(val_to_find).unwrap();
    
    let solution_val: Vec<(&usize,&usize)> = map.iter()
        .filter(|&(_,pos)| *pos == pos_val_to_find+1)
        .collect();

    *solution_val[0].0
}