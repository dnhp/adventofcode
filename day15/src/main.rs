struct Generator {
    factor: u64,
    modulus: u64,
    current_val: u64,
}

impl Generator {
    fn new (factor: u64, modulus: u64, seed: u64) -> Generator {
        Generator {
            factor,
            modulus,
            current_val: seed,
        }
    }
}

impl Iterator for Generator {
    type Item = u64;
    fn next (&mut self) -> Option<u64> {
        self.current_val = (self.current_val * self.factor) % self.modulus;
        Some(self.current_val)
    }
}

fn main() {

    let gen1 = Generator::new(16807, 2147483647, 783);
    let gen2 = Generator::new(48271, 2147483647, 325);

    let n_matches_p1 = gen1.zip(gen2).take(40_000_000)
        .filter(|&(g1,g2)| (g1 & 0xFFFF) == (g2 & 0xFFFF))
        .count();

    println!("Matches [part1]: {:?}", n_matches_p1);

    let gen1 = Generator::new(16807, 2147483647, 783);
    let gen2 = Generator::new(48271, 2147483647, 325);

    let n_matches_p2 = gen1.filter(|&g1| g1 & 3 == 0)
        .zip(gen2.filter(|&g2| g2 & 7 == 0))
        .take(5_000_000)
        .filter(|&(g1,g2)| (g1 & 0xFFFF) == (g2 & 0xFFFF))
        .count();

    println!("Matches [part2]: {:?}", n_matches_p2);
}