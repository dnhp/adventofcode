use std::fs::File;
use std::io::{BufRead,BufReader};
use std::collections::HashMap;

fn main() {

    let input_file = File::open("input.txt").unwrap();
    let buf = BufReader::new(input_file);

    let instructions: Vec<String> = buf.lines()
        .map(|l| l.unwrap())
        .collect();

    //println!("Instr: {:?}", instructions);

    let mut reg_names: Vec<char> = vec![];

    let mut instr_vec: Vec<(String, char, String)> = Vec::new();

    for inst in instructions {
        let mut parts = inst.split_whitespace();

        let instr_tup: (String, char, String) = (

            parts.next().unwrap()
                .to_string(), // opcode

            parts.next().unwrap()
                .chars()
                .nth(0).unwrap(), // register or literal val

            match parts.next() {               // optional third val
                Some(val) => val.to_string(),
                None => "NA".to_string(),
            }
        );

        let reg_char = instr_tup.1;
        if reg_char.is_alphabetic() {
            reg_names.push(reg_char);
        }

        instr_vec.push(instr_tup);    
    }

    reg_names.sort();
    reg_names.dedup();

    let mut registers = HashMap::with_capacity(reg_names.len());
    for reg in &reg_names {
        registers.insert(reg, 0i32  );
    }

    let mut pc = 0i32;
    let mut halt = false;
    let mut last_sound_played = 0;

    while !halt {

        let inst = &instr_vec[pc as usize];

        match &inst.0[..] {// opcode 
            "set" => {
                // Setting register to a value. Assume first arg is
                // always a register letter and never a literal val
                let reg_char = &inst.1;
                let new_val = if let Ok(literal_val) = inst.2.parse::<i32>() {
                    // Is a literal value
                    literal_val
                }
                else {
                    // Is a letter referring to another register. This branch
                    // taken if parse() returns Err(), meaning it's not a number.
                    *registers.get(
                        &inst.2.chars()
                            .nth(0).unwrap())
                        .unwrap()
                };

                let reg = registers.entry(reg_char).or_insert(0);
                *reg = new_val;
                pc += 1;
                println!("SET: Setting register {:?} to {:?}", reg_char, new_val);
            },

            "mul" => {
                // Set register X to product of X and Y. Assume first arg is.
                // always a register letter and never a literal val
                let reg_char = &inst.1;
                let val_in_reg = registers.get(reg_char).unwrap().clone();

                let new_val = if let Ok(literal_val) = inst.2.parse::<i32>() {
                    literal_val
                }
                else { // Is a letter referring to another register
                    *registers.get(
                        &inst.2.chars()
                            .nth(0).unwrap())
                        .unwrap()
                };

                let reg = registers.entry(reg_char).or_insert(0);
                *reg = val_in_reg * new_val;
                pc += 1;
                println!("MUL: Multiplying {:?} and {:?} into {:?}", val_in_reg, new_val, reg_char);
            },

            "jgz" => {
                // Jump with offset Y if X > 0
                let val_if_gz = if inst.1.is_numeric() {
                    // Is a literal value
                    inst.1.to_digit(10).unwrap() as i32
                }
                else { // Is a letter referring to another register.
                    *registers.get(&inst.1).unwrap()
                };

                let offset = if let Ok(literal_val) = inst.2.parse::<i32>() {
                    literal_val
                }
                else {
                    // Is a letter referring to another register. This branch
                    // taken if parse() returns Err(), meaning it's not a number.
                    *registers.get(&inst.2.chars()
                            .nth(0)
                            .unwrap())
                        .unwrap()
                };

                if val_if_gz > 0 {
                    pc += offset;
                    println!("JGZ: Jumping with offset {:?}", offset);
                }
                else {
                    pc += 1;
                    println!("JGZ: Didn't branch");
                }

            }
            _ => {
                halt=true;
                println!("HALTING");
            },
        }
    }
    

}
