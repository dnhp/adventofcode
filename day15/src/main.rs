use std::thread;
use std::sync::{Arc, Mutex};

struct Generator {
    factor: u64,
    modulus: u64,
    seed: u64,
    current_val: u64,
}

impl Generator {
    fn new (factor: u64, modulus: u64, seed: u64) -> Generator {
        Generator {
            factor,
            modulus,
            seed,
            current_val: seed,
        }
    }

    fn reset (&mut self) {
        self.current_val = self.seed;
    }

    fn next (&mut self) -> u64 {
        self.current_val = (self.current_val * self.factor) % self.modulus;
        self.current_val
    }

    fn next_with_limit (&mut self, limit: u64) -> u64 {
        loop {
            let out = self.next();
            if out & (limit-1) == 0 {
                return out;
            }
        }
    }
}

fn main() {

    // Create two generators (factor, modulus, seed)
    let mut gen1 = Generator::new(16807, 2147483647, 783);
    let mut gen2 = Generator::new(48271, 2147483647, 325);

    // Get first 40m results and check lower 16 bits with
    // bitmask
    let mut n_matches_p1 = 0;
    for _ in 0..40_000_000 {
        if (gen1.next() & 0xFFFF) == (gen2.next() & 0xFFFF) {
            n_matches_p1 += 1;
        }
    }
    println!("Number of matches [part1]: {:?}", n_matches_p1);

    gen1.reset();
    gen2.reset();

    // Prepare empty vectors with length 5,000,000 to be passed mutably to
    // new threads.
    let gen1_valid_values = Arc::new(Mutex::new(Vec::with_capacity(5_000_000)));
    let gen2_valid_values = Arc::new(Mutex::new(Vec::with_capacity(5_000_000)));

    // Get additional pointers to memory to be passed to threads. Threads take
    // ownership of these pointers
    let gen1_clone_ptr = gen1_valid_values.clone();
    let gen2_clone_ptr = gen2_valid_values.clone();

    // Spawn two threads, one for each generator.
    // Threads take ownership of generators and drop
    // them.
    let handle1 = thread::spawn(move || {
        let mut gen1_clone_ptr = gen1_clone_ptr.lock().unwrap();
        for _ in 0..5_000_000 {
            gen1_clone_ptr.push(gen1.next_with_limit(4));
        }
    });

    let handle2 = thread::spawn(move || {
        let mut gen2_clone_ptr = gen2_clone_ptr.lock().unwrap();
        for _ in 0..5_000_000 {
            gen2_clone_ptr.push(gen2.next_with_limit(8));
        }
    });

    // Block main thread until both generators have filled
    // their results vectors
    handle1.join().unwrap();
    handle2.join().unwrap();
    
    // Get values in vectors returned from threads
    let g1_vals = &gen1_valid_values.lock().unwrap();
    let g2_vals = &gen2_valid_values.lock().unwrap();

    // Check for matches in lower 16 bits by masking
    // with 0xFFFF and comparing results
    let mut n_matches_p2 = 0;
    for (g1_val,g2_val) in g1_vals.iter().zip(g2_vals.iter()) {
        if (g1_val & 0xFFFF) == (g2_val & 0xFFFF) {
            n_matches_p2 += 1;
        }
    }
    println!("Number of matches [part2]: {:?}", n_matches_p2);
}