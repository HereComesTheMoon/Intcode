use std::collections::VecDeque;
use std::fmt::{Debug, Display, format};
use std::io;
use std::error::Error;

pub type VALUE = i64;

//#[allow(dead_code)]


#[derive(Copy, Clone, Debug)]
pub enum InterpreterError {
    Terminated,
    IoError,
    ParseError,
}

impl Display for InterpreterError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{ File: {}, Line: {} }}", file!(), line!())
    }
}

impl Error for InterpreterError {}



//#[derive(Debug)]
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
    //pub output_stream: Box<dyn io::Write + 'a>,
    //pub input_buffer: Vec<VALUE>,
    pub input_buffer: VecDeque<VALUE>,
    pub last_output: Option<VALUE>,
    pub error: Option<InterpreterError>,
}

impl Default for Interpreter {
    fn default() -> Interpreter {
        Interpreter { code: vec![], ip: 0, param_indices: vec![], finish: false, input_buffer: VecDeque::new(), last_output: None, error: None }
    }
}

impl Debug for Interpreter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s: String = format!("IP: {}, Parameter Indices: {:?}, Input Buffer: {:?}, Last Output: {:?}, Error: {:?}\n", 
                                    self.ip, self.param_indices, self.input_buffer, self.last_output, self.error);
        
        let width = f64::log10(self.code
            .iter()
            .fold(self.code.len() as VALUE, |current, &x| VALUE::max(current, x)) as f64).floor() as usize + 1;
        

        let start = usize::max(self.ip.saturating_sub(7) , 0);
        let end = usize::min(self.ip + 7, self.code.len());

        s += "[";
        for k in start..end {
            s += &*format!("{:>width$},", k);
        }
        s += "]\n[";
        for k in start..end {
            s += &*format!("{:>width$},", self.code[k]);
        }
        s += "]\n";

        //s += "[";
        //for k in 1..self.code.len() {
            //s += &*format!("{:>width$},", k);
        //}
        //s += "\n";
        //for x in self.code.iter() {
            //s += &*format!("{:>width$},", x);
        //}
        //s += "\n";
        writeln!(f, "{}", s)
    }
}

// Idea: The opcodes should only modify Interpreter.code, and they should be the only functions to modify Interpreter.code
// The parser should purely load param_indices, and modify ip .

impl Interpreter {
    pub fn step(&mut self) -> Result<Option<VALUE>, InterpreterError> {
        if self.finish {
            return Err(InterpreterError::Terminated);
        }
        self.last_output = None;
        self.error = None;

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

        if let Some(err) = self.error {
            return Err(err)
        }

        if let Some(output) = self.last_output {
            return Ok(Some(output))
        }

        return Ok(None)
    }

    pub fn step_loop(&mut self) -> Result<Option<VALUE>, InterpreterError> { 
        loop {
            let res = self.step();
            match res {
                Err(_) => { return res; },
                Ok(Some(_)) => { return res; },
                Ok(None) => {},
            }
        }
    }

    pub fn new<'a>(code: Vec<VALUE>, input_buffer: VecDeque<VALUE>) -> Interpreter {
        Interpreter {
            code,
            ip: 0,
            param_indices: vec![],
            finish: false,
            input_buffer,
            last_output: None,
            error: None,
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
    pc.error = Some(InterpreterError::Terminated);
    pc.ip += 0;
}

fn op_add(pc: &mut Interpreter) {
    pc.code[pc.param_indices[2] as usize] = pc.code[pc.param_indices[0] as usize] + pc.code[pc.param_indices[1] as usize];
    pc.ip += 4;
}

fn op_mul(pc: &mut Interpreter) {
    pc.code[pc.param_indices[2] as usize] = pc.code[pc.param_indices[0] as usize] * pc.code[pc.param_indices[1] as usize];
    pc.ip += 4;
}

fn op_in(pc: &mut Interpreter) {
    //FIXME: Input buffer should be a queue! FIFO, not FILO
    print!("Reading input... ");
    let mut input = String::new();

    if let Some(val) = pc.input_buffer.pop_front() {
        println!("{}.", val);
        pc.code[pc.param_indices[0]] = val;
        pc.ip += 2;
        return
    }

    println!("Input buffer empty. Use stdin. Waiting for input: ");
    if let Err(e) = io::stdin().read_line(&mut input) {
        println!("Error: Interpreter failed to read input: {}", e);
        pc.error = Some(InterpreterError::IoError);
        return
    } 

    if let Ok(num) = input.parse::<VALUE>() {
        println!("{}.", num);
        pc.code[pc.param_indices[0]] = num;
        pc.ip += 2;
        return
    } 

    println!("Error: Interpreter failed to parse input.");
    pc.error = Some(InterpreterError::ParseError);
}

fn op_out(pc: &mut Interpreter) {
    let res = pc.code[pc.param_indices[0]];
    println!("OUTPUT: {}", res);
    println!("{:?}", pc);

    let temp_ip = pc.ip;
    pc.ip = pc.code[pc.param_indices[0]] as usize;
    println!("{:?}", pc);
    pc.ip = temp_ip;

    pc.last_output = Some(res);
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

