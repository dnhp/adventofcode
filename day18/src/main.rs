use std::fs::File;
use std::io::{BufRead,BufReader};
use std::collections::HashMap;
use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
use std::thread;

struct Program {
    instructions: Vec<(String, char, String)>,
    pc: i32,
    registers: HashMap<char, i64>,
    halt: bool,
    sender: Sender<i64>,
    receiver: Receiver<i64>,
    vals_sent: Vec<i64>,
    prog_id: i64,
}

impl Program {
    fn new(
        instructions: Vec<(String, char, String)>,
        reg_names: Vec<char>,
        reg_p_val: i64,
        sender: Sender<i64>, 
        receiver: Receiver<i64>) -> Program {
        
        let mut reg_hashmap = HashMap::with_capacity(reg_names.len());
        for reg in reg_names {
            reg_hashmap.insert(reg, 0i64);
        }

        reg_hashmap.insert('p', reg_p_val);

        Program {
            instructions,
            pc: 0,
            registers: reg_hashmap,
            halt: false,
            sender,
            receiver,
            vals_sent: vec![],
            prog_id: reg_p_val,
        }
    }

    fn run (&mut self) {

        while !self.halt {

            if self.pc > self.instructions.len() as i32 || self.pc < 0 {
                println!("Program jumped off end");
                self.halt = true;
                break;
            }
            let pc = self.pc;
            let inst = &self.instructions[pc as usize];

            match &inst.0[..] {// opcode 
                "set" => {
                    // Setting register to a value. Assume first arg is
                    // always a register letter and never a literal val
                    let reg_char = &inst.1;
                    let new_val = if let Ok(literal_val) = inst.2.parse::<i64>() {
                        literal_val
                    }
                    else {
                        // Is a letter referring to another register. This branch
                        // taken if parse() returns Err(), meaning it's not a number.
                        *self.registers.get(
                            &inst.2.chars()
                                .nth(0).unwrap())
                            .unwrap()
                    };

                    let reg = self.registers.entry(*reg_char).or_insert(0);
                    *reg = new_val;
                    self.pc += 1;
                },

                "mul" => {
                    // Set register X to product of X and Y. Assume first arg is.
                    // always a register letter and never a literal val
                    let reg_char = &inst.1;
                    let val_in_reg = self.registers.get(reg_char).unwrap().clone();

                    let new_val = if let Ok(literal_val) = inst.2.parse::<i64>() {
                        literal_val
                    }
                    else { // Is a letter referring to another register
                        *self.registers.get(
                            &inst.2.chars()
                                .nth(0).unwrap())
                            .unwrap()
                    };

                    let reg = self.registers.entry(*reg_char).or_insert(0);
                    *reg = val_in_reg * new_val;
                    self.pc += 1;
                },

                "jgz" => {
                    // Jump with offset Y if X > 0
                    let val_if_gz = if inst.1.is_numeric() { // Is a literal value
                        inst.1.to_digit(10).unwrap() as i64
                    }
                    else { // Is a letter referring to another register.
                        *self.registers.get(&inst.1).unwrap()
                    };

                    let offset = if let Ok(literal_val) = inst.2.parse::<i64>() {
                        literal_val
                    }
                    else {
                        // Is a letter referring to another register. This branch
                        // taken if parse() returns Err(), meaning it's not a number.
                        *self.registers.get(&inst.2.chars()
                                .nth(0)
                                .unwrap())
                            .unwrap()
                    };

                    if val_if_gz > 0 {
                        self.pc += offset as i32;
                    }
                    else {
                        self.pc += 1;
                    }
                },
                
                "add" => {
                    // Increment register X by Y. Assume first arg is.
                    // always a register letter and never a literal val
                    let reg_char = &inst.1;
                    let val_in_reg = self.registers.get(reg_char).unwrap().clone();

                    let new_val = if let Ok(literal_val) = inst.2.parse::<i64>() {
                        literal_val
                    }
                    else { // Is a letter referring to another register
                        *self.registers.get(
                            &inst.2.chars()
                                .nth(0).unwrap())
                            .unwrap()
                    };

                    let reg = self.registers.entry(*reg_char).or_insert(0);
                    *reg = val_in_reg + new_val;
                    self.pc += 1;
                },

                "mod" => {

                    // Set register X to X % Y. Assume first arg is.
                    // always a register letter and never a literal val
                    let reg_char = &inst.1;
                    let val_in_reg = self.registers.get(reg_char).unwrap().clone();

                    let new_val = if let Ok(literal_val) = inst.2.parse::<i64>() {
                        literal_val
                    }
                    else { // Is a letter referring to another register
                        *self.registers.get(
                            &inst.2.chars()
                                .nth(0).unwrap())
                            .unwrap()
                    };

                    let reg = self.registers.entry(*reg_char).or_insert(0);
                    *reg = val_in_reg % new_val;
                    self.pc += 1;
                },

                "snd" => {

                    let val_to_send = if inst.1.is_numeric() {
                        // Is a literal value
                        inst.1.to_digit(10).unwrap() as i64
                    }
                    else { // Is a letter referring to another register.
                        *self.registers.get(&inst.1).unwrap()
                    };

                    self.sender.send(val_to_send).unwrap();
                    self.vals_sent.push(val_to_send);

                    println!("Prog ID: {:?} vals sent: : {:?}", self.prog_id, self.vals_sent.len());

                    self.pc += 1;
                },

                "rcv" => {

                    let received_val = self.receiver.recv().unwrap();

                    let reg_char = &inst.1;
                    let reg = self.registers.entry(*reg_char).or_insert(0);
                    *reg = received_val;
                    
                    self.pc += 1;
                },

                _ => {
                    self.halt=true;
                    panic!("HALTING at unknown opcode");
                },
            }
        }
    }
}


fn main() {

    // Begin by reading and parsing the instructions into
    // a vector to be passed to the program
    let input_file = File::open("input.txt").unwrap();
    let buf = BufReader::new(input_file);

    let instructions: Vec<String> = buf.lines()
        .map(|l| l.unwrap())
        .collect();

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

        // Get list of register names so it can
        // be passed to the program to initialise
        // hashmap representing the registers
        let reg_char = instr_tup.1;
        if reg_char.is_alphabetic() {
            reg_names.push(reg_char);
        }
        instr_vec.push(instr_tup);    
    }

    reg_names.sort();
    reg_names.dedup();

    // Create transmitters and receivers to go in each program
    let (tx_from_1, rx_from_1): (Sender<i64>, Receiver<i64>) = mpsc::channel();
    let (tx_from_2, rx_from_2): (Sender<i64>, Receiver<i64>) = mpsc::channel();

    let prog_id_1 = 0;
    let prog_id_2 = 1;

    // Initialise two programs
    let mut prog1 = Program::new(instr_vec.clone(), reg_names.clone(), prog_id_1, tx_from_1, rx_from_2);
    let mut prog2 = Program::new(instr_vec.clone(), reg_names.clone(), prog_id_2, tx_from_2, rx_from_1);

    // Give one program to each thread and run them
    let h1 = thread::spawn(move || {
        prog1.run();
    });
    let h2 = thread::spawn(move || {
        prog2.run();
    });

    h1.join().unwrap();
    h2.join().unwrap();
}
