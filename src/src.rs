use std::io;
use std::process;

pub type VALUE = i64;

//#[derive(Debug)]
pub struct Interpreter<'a> {
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
    pub output_stream: Box<dyn io::Write + 'a>,
    pub input_stream: Box<dyn io::BufRead + 'a>,
    //pub output_stream: &'a dyn io::Write,
}

impl<'a> Default for Interpreter<'a> {
    fn default() -> Interpreter<'a> {
        Interpreter { code: vec![], ip: 0, param_indices: vec![], finish: false, input_stream: Box::new(io::stdin().lock()), output_stream: Box::new(io::stdout()) }
    }
}

// Idea: The opcodes should only modify Interpreter.code, and they should be the only functions to modify Interpreter.code
// The parser should purely load param_indices, and modify ip .

impl Interpreter<'_> {
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

    pub fn new<'a>(code: Vec<VALUE>, output_stream: Box<dyn io::Write>, input_stream: Box<dyn io::BufRead>) -> Interpreter<'a> {
        //let out: Box<dyn io::Write + 'a>,
        //input_stream: Box<dyn io::BufRead + 'a>,

        Interpreter {
            code,
            ip: 0,
            param_indices: vec![],
            finish: false,
            output_stream, 
            input_stream,
        }
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
    print!("Reading input... ");
    let mut input = String::new();
    loop {
        match (*pc.input_stream).read_line(&mut input) {
            Ok(_) => {},
            Err(e) => {
                println!("Error: Interpreter failed to read input: {}", e);
                process::abort(); // pie_flavor: abort() better than panic!() here
            },
        }
        let res = input.trim().parse::<VALUE>();
        if let Ok(num) = res {
            pc.code[pc.param_indices[0]] = num;
            pc.ip += 2;
            println!("Read input: {}", num);
            break;
        } else {
            input.clear();
        }
    }
}

fn op_out(pc: &mut Interpreter) {
    println!("OUTPUT: {}", pc.code[pc.param_indices[0]]);
    if let Err(e) = (*pc.output_stream).
        write_fmt(format_args!("{}\n", pc.code[pc.param_indices[0]]))
            //write_fmt(format_args!("OUTPUT: {}\n", pc.code[pc.param_indices[0]])) {
            .and_then( |_| (*pc.output_stream).flush() ) {
        println!("Error: Interpreter failed to send output: {}", e);
        process::abort();
    };

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

