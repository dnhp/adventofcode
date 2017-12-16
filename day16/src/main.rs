extern crate num;

use std::fs::File;
use std::io::prelude::*;
use std::collections::VecDeque;
use num::integer;

struct ProgList {
    prog_state: VecDeque<char>,
}

impl ProgList {
    fn new (prog_state: &VecDeque<char>) -> ProgList {
        ProgList {
            prog_state: prog_state.clone(),
        }
    }

    fn execute_instruction (&mut self, instr: &str) {
        match instr.chars().nth(0).unwrap(){
            's' => self.spin(&instr[1..]),
            'x' => self.exchange(&instr[1..]),
            'p' => self.partner(&instr[1..]),
            _ => panic!("Unrecognised instruction."),
        }
    }

    fn spin (&mut self, instr: &str) {
        let n_spin: usize = instr.parse().unwrap();
        for _ in 0..n_spin {
            let tmp = self.prog_state.pop_back().unwrap();
            self.prog_state.push_front(tmp);
        }
    }

    fn exchange (&mut self, instr: &str) {
        let pos: Vec<usize> = instr.split('/')
            .map(|cs| cs.parse().unwrap())
            .collect();
        self.prog_state.swap(pos[0],pos[1]);
    }

    fn partner (&mut self, instr: &str) {
        let p1 = instr.chars().nth(0).unwrap();
        let p2 = instr.chars().nth(2).unwrap();
        let pos1 = self.prog_state.iter().position(|&c| c==p1).unwrap();
        let pos2 = self.prog_state.iter().position(|&c| c==p2).unwrap();
        self.prog_state.swap(pos1,pos2);
    }
}


fn main() {

    let mut f = File::open("./input.txt").unwrap();
    let mut contents = String::new();
    f.read_to_string(&mut contents).unwrap();
    
    let instrs: Vec<&str> = contents.trim()
                            .split(',')
                            .collect();

    let prog_names = VecDeque::from(vec!['a','b','c','d','e','f','g','h','i','j','k','l','m','n','o','p']);

    /* Part 1 */
    // Create and initialise struct for program state
    let mut state = ProgList::new(&prog_names);

    // Run all instructions in input
    for inst in &instrs {
        state.execute_instruction(inst);
    }

    println!("State after 1 iteration [part1]: {:?}", state.prog_state.iter().collect::<String>());

    /* Part 2 */
    // Separate instructions intp partner instructions and spin/exchange instructions.
    // We want to run them separately as the two types of permutation commute. We want
    // to find the permutation period of both the partner instructions and the spin/
    // exchange instructions. The total period of the full permutation is then the lowest
    // common multiple of each period. We then take the result of 1 billion modulo this
    // period to find how many additional permutation cycles must be applied to get the
    // solution.

    // Seperate the two instruction sets by checking if the letter 'p' is present.
    // The letter 'p' does not occur outside of partner instructions.
    let (part_instr, spin_ex_instr): (Vec<&str>,Vec<&str>) = instrs.iter()
        .partition(|inst| inst.contains('p'));

    // Initialise a new program state, and make a vector to hold the history
    // of the state.
    let mut state = ProgList::new(&prog_names);
    let mut state_history_partner: Vec<VecDeque<char>> = Vec::new();
    state_history_partner.push(state.prog_state.clone());


    // First, apply the partner instructions until we find the period of the permutation.
    let mut partner_period = 0;
    loop {
        for instr in &part_instr {
            state.execute_instruction(instr);
        }
        partner_period += 1;
        if state_history_partner.iter().any(|vd| *vd == state.prog_state) {
            break;
        }
        state_history_partner.push(state.prog_state.clone());
    }

    // Program sequence has been reset, so now apply all exchange/spin instructions
    // to find period.
    let mut state_history_spin_ex: Vec<VecDeque<char>> = Vec::new();
    state_history_spin_ex.push(state.prog_state.clone());

    let mut spin_ex_period = 0;
    loop {
        for instr in &spin_ex_instr {
            state.execute_instruction(instr);
        }
        spin_ex_period += 1;
        if state_history_spin_ex.iter().any(|vd| *vd == state.prog_state) {
            break;
        }
        state_history_spin_ex.push(state.prog_state.clone());
    }

    // Get the lowest commom multiple of the two periods
    let full_permutation_period = integer::lcm(partner_period, spin_ex_period);

    // Now calculate 1 billion modulo full_permutation_period. Remainder is how
    // many cycles will have been executed since the last repeat. This is how
    // many we need to appy to get the answer.
    let n_cycles_remaining = 1_000_000_000 % full_permutation_period;

    // Run this number of cycles to get program state after 1bn iterations.
    for _ in 0..n_cycles_remaining {
        for instr in &part_instr {
            state.execute_instruction(instr);
        }
        for instr in &spin_ex_instr {
            state.execute_instruction(instr);
        }
    }
    println!("State after 1bn iterations [part2]: {:?}", state.prog_state.iter().collect::<String>());
}