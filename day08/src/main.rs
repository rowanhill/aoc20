#![feature(str_split_once)]

use std::io::{BufReader, BufRead};
use std::fs::File;

enum Instruction {
    Acc(i32),
    Jmp(i32),
    Nop(i32),
}
impl Instruction {
    fn from_str(def: &str) -> Instruction {
        let (name, num_def) = def.split_once(" ")
            .expect("Could not split instruction definition on space");
        match name {
            "acc" => Instruction::Acc(num_def.parse().unwrap()),
            "jmp" => Instruction::Jmp(num_def.parse().unwrap()),
            "nop" => Instruction::Nop(num_def.parse().unwrap()),
            _ => panic!("Unknown instruction name")
        }
    }
}

struct Computer {
    accumulator: i32,
}
impl Computer {
    fn new() -> Computer {
        Computer { accumulator: 0 }
    }

    fn run_program_until_loop(&mut self, program: &Vec<Instruction>) {
        let mut instr_index = 0;
        let mut visited = vec![false; program.len()];

        while visited[instr_index] == false {
            let mut jmp = 1;
            let instr = &program[instr_index];
            match instr {
                &Instruction::Acc(inc) => self.accumulator += inc,
                &Instruction::Jmp(dist) => jmp = dist,
                &Instruction::Nop(_) => {}
            }
            visited[instr_index] = true;
            instr_index = (instr_index as i32 + jmp) as usize;
        }
    }

    fn run_program(&mut self, program: &Vec<Instruction>) -> Option<i32> {
        let mut instr_index = 0;
        let mut visited = vec![false; program.len()];

        while instr_index < program.len() {
            if visited[instr_index] {
                return None;
            }

            let mut jmp = 1;
            let instr = &program[instr_index];
            match instr {
                &Instruction::Acc(inc) => self.accumulator += inc,
                &Instruction::Jmp(dist) => jmp = dist,
                &Instruction::Nop(_) => {}
            }
            visited[instr_index] = true;
            instr_index = (instr_index as i32 + jmp) as usize;
        }

        Some(self.accumulator)
    }
}

fn main() {
    let reader = BufReader::new(File::open("input")
        .expect("Cannot open input"));

    let mut program: Vec<Instruction> = reader.lines().map(|l| {
        let line = l.unwrap();
        Instruction::from_str(&line)
    }).collect();

    let mut computer = Computer::new();
    computer.run_program_until_loop(&program);
    println!("Part 1: {}", computer.accumulator);

    for i in 0..program.len() {
        match program[i] {
            Instruction::Acc(_) => {}
            Instruction::Jmp(x) => {
                let old = program.remove(i);
                let new = Instruction::Nop(x);
                program.insert(i, new);

                let mut c = Computer::new();
                let result = c.run_program(&program);

                if let Some(acc) = result {
                    println!("Part 2: {}", acc);
                }

                program.remove(i);
                program.insert(i, old);
            },
            Instruction::Nop(x) => {
                let old = program.remove(i);
                let new = Instruction::Jmp(x);
                program.insert(i, new);

                let mut c = Computer::new();
                let result = c.run_program(&program);

                if let Some(acc) = result {
                    println!("Part 2: {}", acc);
                }

                program.remove(i);
                program.insert(i, old);
            }
        }
    };
}
