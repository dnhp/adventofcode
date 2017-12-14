use std::fmt;

pub fn knot_hash (
    input_string: &str,
    output_format: char
    ) -> String {

    let mut data_ascii: Vec<u8> = input_string.trim()
                                    .chars()
                                    .map(|c| c as u8)
                                    .collect();

    let extra_data: Vec<u8> = vec![17, 31, 73, 47, 23];
    for val in extra_data {
        data_ascii.push(val);
    }
    
    let n_el = 256;
    let num_hash_rounds = 64;
    let dense_hash_block_size = 16;

    let init_num_list: Vec<u8> = (0..n_el).map(|n| n as u8)
                                    .collect();

    let sparse_hash = get_sparse_hash(&init_num_list, &data_ascii, num_hash_rounds, n_el);

    let dense_hash = get_dense_hash(&sparse_hash, dense_hash_block_size);
    // format dense hash vector into hex
    let mut dense_hash_str = String::new();
    match output_format {
        'x' => {
            for byte in dense_hash {
                fmt::write(&mut dense_hash_str, format_args!("{:08x}", byte)).unwrap();
            }
        },
        'b' => {
            for byte in dense_hash {
                fmt::write(&mut dense_hash_str, format_args!("{:08b}", byte)).unwrap();
            }
        },
        _ => panic!("invalid flag"),
    }

    dense_hash_str
}

fn get_dense_hash (
    sparse_hash: &Vec<u8>,
    block_size: usize) -> Vec<u8> {

    sparse_hash.chunks(block_size)
        .map(|chunk| chunk.iter().fold(0u8, |acc, &val| acc ^ val))
        .collect()
}

fn get_sparse_hash (
    init_num_list: &Vec<u8>,
    data_ascii: &Vec<u8>,
    num_hash_rounds: usize,
    n_el: usize) -> Vec<u8> {

    let mut c_pos: usize = 0;
    let mut skip: usize = 0;

    let mut hashed_list: Vec<u8> = init_num_list.clone();

    for _ in 0..num_hash_rounds {
        run_single_hash_round(&mut hashed_list, data_ascii, &mut c_pos, &mut skip, n_el);
    }
    hashed_list
}

fn run_single_hash_round (
    num_list: &mut Vec<u8>,
    lengths: &Vec<u8>,
    c_pos: &mut usize,
    skip: &mut usize,
    n_el: usize) {

    for in_len in lengths {
        if *in_len > 1 {
            mutate_num_list(num_list, *in_len as usize, *c_pos, n_el);
        }
        *c_pos += *in_len as usize + *skip;
        *c_pos = *c_pos % n_el;
        *skip += 1;
    }
}

fn mutate_num_list (
    num_list: &mut Vec<u8>,
    in_len: usize,
    c_pos: usize,
    n_el: usize) {

    let swap_range: Vec<usize> = (c_pos..c_pos+in_len).map(|x| x%n_el).collect();

    let mut vec_to_rev: Vec<u8> = Vec::with_capacity(swap_range.len());

    // push values onto the vector like a stack - can pop them off in reverse order
    for ind in &swap_range {
        vec_to_rev.push(num_list[*ind]);
    }

    // pop values off vector, effectively reversing the order
    for swap_ind in &swap_range {
        num_list[*swap_ind] = vec_to_rev.pop().unwrap();
    }
}