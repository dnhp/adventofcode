use std::thread;
use std::sync::{Arc, Mutex};

fn main() {

    let factor_gen1: u64 = 16807;
    let factor_gen2: u64 = 48271;

    let modulus: u64 = 2147483647;

    let seed_gen1: u64 = 783;
    let seed_gen2: u64 = 325;

    let num_pairs_p1 = 40_000_000;

    let mut n_matches: usize = 0;

    let mut current_val_gen1 = seed_gen1;
    let mut current_val_gen2 = seed_gen2;

    let bitmask = 65535; // 2^16 - 1, all lower 16 bits set for unsigned int
    
    for _ in 0..num_pairs_p1 {
        generate_next_val_p1(&mut current_val_gen1, factor_gen1, modulus);
        generate_next_val_p1(&mut current_val_gen2, factor_gen2, modulus);
        if (current_val_gen1 & bitmask) == (current_val_gen2 & bitmask) {
            n_matches += 1;
        }
    }
    println!("Number of matches [part1]: {:?}", n_matches);

    //let num_valid_values: usize = 5_000_000;
    let num_pairs_p2: usize = 5_000_000;

    // Reset generators
    current_val_gen1 = seed_gen1;
    current_val_gen2 = seed_gen2;

    // Prepare empty vectors with length 5,000,000 to be passed mutably to
    // new threads.
    let gen1_valid_values = Arc::new(Mutex::new(Vec::with_capacity(num_pairs_p2)));
    let gen2_valid_values = Arc::new(Mutex::new(Vec::with_capacity(num_pairs_p2)));

    // Get additional pointers to memory to be passed to threads. Threads take
    // ownership of these pointers
    let g1_clone = gen1_valid_values.clone();
    let g2_clone = gen2_valid_values.clone();

    let gen1_multipleof = 4;
    let gen2_multipleof = 8;
    
    // Spawn two threads, each handling one generator
    let handle1 = thread::spawn(move || {
        let mut g1_clone = g1_clone.lock().unwrap();
        for _ in 0..num_pairs_p2 {
            g1_clone.push(
                generate_next_val_p2(&mut current_val_gen1,
                    factor_gen1,
                    modulus,
                    gen1_multipleof));
        }
    });

    let handle2 = thread::spawn(move || {
        let mut g2_clone = g2_clone.lock().unwrap();
        for _ in 0..num_pairs_p2 {
            g2_clone.push(
                generate_next_val_p2(&mut current_val_gen2,
                    factor_gen2,
                    modulus,
                    gen2_multipleof));
        }
    });

    // Block main thread until both generators have filled
    // their results vectors
    handle1.join().unwrap();
    handle2.join().unwrap();

    // Get values in vectors returned from threads
    let g1_vals = gen1_valid_values.lock().unwrap();
    let g2_vals = gen2_valid_values.lock().unwrap();

    // Check for matches in lower 16 bits by masking
    // with 0xFFFF and comparing results
    let mut n_matches_p2 = 0;

    for el in g1_vals.iter().zip(g2_vals.iter()) {
        if (el.0 & bitmask) == (el.1 & bitmask) {
            n_matches_p2 += 1;
        }
    }
    println!("Number of matches [part2]: {:?}", n_matches_p2);
}

fn generate_next_val_p1 (
    current_val: &mut u64,
    factor: u64,
    modulus: u64) {

    *current_val = (*current_val * factor) % modulus;
}

fn generate_next_val_p2 (
    current_val: &mut u64,
    factor: u64,
    modulus: u64,
    multiple_of: u64) -> u64 {

    // Checks if generated value is a multiple of multiple_of
    // by masking with multiple_of-1 and comparing with zero.
    // As in this case multiple_of is 4 or 8 (powers of two) we
    // can just check whether the bits below 4 or 8 are set.
    // E.g. to check if multiple of 8, mask with 0b0111.
    loop {
        generate_next_val_p1(current_val, factor, modulus);
        if *current_val & (multiple_of-1) == 0 {
            return *current_val
        }
    }
}