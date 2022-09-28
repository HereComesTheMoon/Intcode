use std::io;

pub type VALUE = i64;

#[derive(Debug)]
pub struct Interpreter {
    /// The underlying code of the program.
    pub code: Vec<VALUE>,
    /// The instruction pointer.
    pub ip: usize,
    /// Indicates whether the ip should be moved after the current instruction.
    //pub move_ip: bool,
    /// The relative base register.
    //pub rel_base: i64,
    /// The arguments for the current instruction.
    pub param_indices: Vec<usize>,
    /// Indicates whether the program is finished.
    pub finish: bool,
}

// Idea: The opcodes should only modify Interpreter.code, and they should be the only functions to modify Interpreter.code
// The parser should purely load param_indices, and modify ip .

impl Interpreter {
    pub fn step(&mut self) {
        if self.finish {
            return
        }
        let next_code = self.code[self.ip];
        let next_instruction = &OPCODES[((next_code % 100) % 99) as usize];

        self.param_indices = (0..next_instruction.number_parameters)
            .map(|k| match (next_code / 10i64.pow(2+k as u32)) % 10 {
                0 => { self.code[self.ip + 1 + k] as usize },
                1 => { self.ip + 1 + k },
                _ => unreachable!(),
            })
            .collect();
        (next_instruction.func)(self);
    }
}

// Warning: Order matters
const OPCODES: [Instruction; 9] = [
    Instruction { name: "halt", opcode: 99, func: op_halt, number_parameters: 0 },
    Instruction { name: "add", opcode: 1, func: op_add, number_parameters: 3 },
    Instruction { name: "multiply", opcode: 2, func: op_mul, number_parameters: 3 },
    Instruction { name: "input", opcode: 3, func: op_in, number_parameters: 1 },
    Instruction { name: "output", opcode: 4, func: op_out, number_parameters: 1 },
    Instruction { name: "jump-if-true", opcode: 5, func: op_jit, number_parameters: 2 },
    Instruction { name: "jump-if-false", opcode: 6, func: op_jif, number_parameters: 2 },
    Instruction { name: "less than", opcode: 7, func: op_lt, number_parameters: 3 },
    Instruction { name: "equals", opcode: 8, func: op_eq, number_parameters: 3 },
];

const MAX_PARAMETERS: usize = 3;

#[allow(dead_code)]
pub struct Instruction {
    opcode: u8,
    name: &'static str,
    func: fn(&mut Interpreter),
    number_parameters: usize,
}

fn op_halt(pc: &mut Interpreter) {
    pc.finish = true;
    pc.ip += 0;
}

fn op_add(pc: &mut Interpreter) {
    //let target: usize = pc.code[pc.ip + 3] as usize;
    //pc.code[target] = pc.code[pc.code[pc.ip + 1] as usize] + pc.code[pc.code[pc.ip + 2] as usize];
    //println!("{} = {} + {}", pc.code[pc.param_indices[2] as usize] ,pc.code[pc.param_indices[0] as usize] , pc.code[pc.param_indices[1] as usize]);
    pc.code[pc.param_indices[2] as usize] = pc.code[pc.param_indices[0] as usize] + pc.code[pc.param_indices[1] as usize];
    pc.ip += 4;
}

fn op_mul(pc: &mut Interpreter) {
    //let target: usize = pc.code[pc.ip + 3] as usize;
    //pc.code[target] = pc.code[pc.code[pc.ip + 1] as usize] * pc.code[pc.code[pc.ip + 2] as usize];
    pc.code[pc.param_indices[2] as usize] = pc.code[pc.param_indices[0] as usize] * pc.code[pc.param_indices[1] as usize];
    pc.ip += 4;
}

fn op_in(pc: &mut Interpreter) {
    println!("GET USER INPUT:");
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => {},
        Err(_) => { println!("ERROR!") },
    }
    pc.code[pc.param_indices[0]] = input.trim().parse().unwrap();
    pc.ip += 2;
}

fn op_out(pc: &mut Interpreter) {
    println!("OUTPUT: {}", pc.code[pc.param_indices[0]]);
    pc.ip += 2;
}

fn op_jit(pc: &mut Interpreter) {
    if pc.code[pc.param_indices[0]] != 0 {
        pc.ip = pc.code[pc.param_indices[1]] as usize;
    } else {
        pc.ip += 3;
    }
}

fn op_jif(pc: &mut Interpreter) {
    if pc.code[pc.param_indices[0]] == 0 {
        pc.ip = pc.code[pc.param_indices[1]] as usize;
    } else {
        pc.ip += 3;
    }
}

fn op_lt(pc: &mut Interpreter) {
    pc.code[pc.param_indices[2]] = (pc.code[pc.param_indices[0]] < pc.code[pc.param_indices[1]]) as VALUE;
    pc.ip += 4;
}

fn op_eq(pc: &mut Interpreter) {
    pc.code[pc.param_indices[2]] = (pc.code[pc.param_indices[0]] == pc.code[pc.param_indices[1]]) as VALUE;
    pc.ip += 4;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_first_example() {
        let code = vec![1,9,10,3,2,3,11,0,99,30,40,50];
        let mut pc: Interpreter = Interpreter { code, ip: 0, param_indices: vec![], finish: false };

        while !pc.finish {
            println!("{:?}", pc);
            pc.step();
        }
        let wanted = vec![3500,9,10,70, 2,3,11,0, 99, 30,40,50];

        assert_eq!(pc.code, wanted);
    }

}
