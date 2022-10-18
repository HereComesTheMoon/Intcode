use std::collections::VecDeque;
use std::fmt::{Debug, Display};
use std::error::Error;

// TODO: Decide if want to keep this, or just make everything i64 from the get-go
pub type VALUE = i64;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum InterpreterError {
    Terminated,
    NoInputError,
    InvalidOpCode,
    InvalidParameters,
    Overflow,
    JumpOutOfBounds,
    OutOfMemory,
}

impl Display for InterpreterError {
    // FIXME: Rewrite the fmt here.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            InterpreterError::Terminated => write!(f, "Terminated"),
            InterpreterError::NoInputError => write!(f, "No input"),
            InterpreterError::InvalidOpCode => write!(f, "Invalid OpCode"),
            InterpreterError::Overflow => write!(f, "Overflow"),
            InterpreterError::JumpOutOfBounds => write!(f, "Jump out of bounds"),
            InterpreterError::OutOfMemory => write!(f, "Jump out of bounds"),
            InterpreterError::InvalidParameters => write!(f, "Invalid parameters"),
        }
    }
}

impl Error for InterpreterError {}



//#[derive(Debug)]
pub struct Interpreter {
    /// The underlying code of the program.
    code: Vec<VALUE>,
    /// The instruction pointer.
    ip: usize,
    /// Current relative base for relative base mode
    relative_base: isize,
    /// The arguments for the current instruction.
    param_indices: Vec<usize>,
    /// Indicates whether the program is finished.
    pub finish: bool,
    /// Input buffer
    pub input_buffer: VecDeque<VALUE>,
    /// The last valid output. 
    pub last_output: Option<VALUE>,
}

impl Default for Interpreter {
    fn default() -> Interpreter {
        Interpreter { code: vec![], ip: 0, relative_base: 0, param_indices: vec![], finish: false, input_buffer: VecDeque::new(), last_output: None }
    }
}

impl Debug for Interpreter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s: String = format!("IP: {}, Parameter Indices: {:?}, Input Buffer: {:?}, Last Output: {:?}\n", 
                                    self.ip, self.param_indices, self.input_buffer, self.last_output);
        
        let start = usize::max(usize::saturating_sub(self.ip, 12) , 0);
        let end = usize::min(usize::saturating_add(self.ip, 12), self.code.len());

        let width = f64::log10(self.code[start..end]
            .iter()
            .fold(end as VALUE, |current, &x| VALUE::max(current, x)) as f64).floor() as usize + 1;
        

        s += "[";
        for k in start..end {
            s += &*format!("{:>width$},", k);
        }
        s += "]\n[";
        for k in start..end {
            s += &*format!("{:>width$},", self.code[k]);
        }
        s += "]\n";
        writeln!(f, "{}", s)
    }
}

impl Interpreter {
    pub fn step(&mut self) -> Result<Option<VALUE>, InterpreterError> {
        if self.finish {
            return Err(InterpreterError::Terminated);
        }
        self.last_output = None;

        let next_code = match self.code.get(self.ip) {
            None => return Err(InterpreterError::OutOfMemory),
            Some(val) => val,
        };

        // Ensures that an opcode 0 is an error, distinguishes from 99
        if next_code % 100 == 0 {
            return Err(InterpreterError::InvalidOpCode);
        }

        let next_instruction = match OPCODES.get(((next_code % 100) % 99) as usize) {
            None => return Err(InterpreterError::InvalidOpCode),
            Some(val) => val,
        };

        let mut wrong_parameters = false;

        self.param_indices = (0..next_instruction.number_parameters)
            .map(|k| match (next_code / 10i64.pow(2+k as u32)) % 10 {
                0 => { self.code[self.ip + 1 + k] as usize },
                1 => { self.ip + 1 + k },
                2 => { (self.relative_base + self.code[self.ip + 1 + k] as isize) as usize },
                _ => { wrong_parameters = true; 0 }
            })
            .collect();

        if wrong_parameters {
            return Err(InterpreterError::InvalidParameters);
        }

        if next_code / 10i64.pow(2+next_instruction.number_parameters as u32) != 0 {
            return Err(InterpreterError::InvalidParameters);
        }

        // immediate mode, never to be used for writing
        if next_code / 10i64.pow(4) == 1 {
            return Err(InterpreterError::InvalidParameters);
        }

        if self.param_indices.iter().any(|x| !(0..self.code.len()).contains(x)) {
            return Err(InterpreterError::InvalidParameters);
        }


        if let Some(e) = (next_instruction.func)(self) {
            Err(e)
        } else {
            Ok(self.last_output)
        }
    }

    pub fn step_loop(&mut self) -> Result<VALUE, InterpreterError> { 
        loop {
            let res = self.step();
            match res {
                Err(e) => { return Err(e); },
                Ok(Some(val)) => { return Ok(val); },
                Ok(None) => {},
            }
        }
    }

    pub fn new<'a>(mut code: Vec<VALUE>, input_buffer: VecDeque<VALUE>) -> Interpreter {
        code.extend(vec![0i64; 9*code.len()]); // Ensure starting memory is large enough. The
                                               // intcode specification does not specify an exact
                                               // size beyond "several times the size of the
                                               // starting memory"
        Interpreter {
            code,
            ip: 0,
            relative_base: 0,
            param_indices: vec![],
            finish: false,
            input_buffer,
            last_output: None,
        }
    }
}

// Warning: Quasi-jump table, Order of items important. 
const OPCODES: [Instruction; 10] = [
    Instruction { name: "halt", opcode: 99, func: op_halt, number_parameters: 0 },
    Instruction { name: "add", opcode: 1, func: op_add, number_parameters: 3 },
    Instruction { name: "multiply", opcode: 2, func: op_mul, number_parameters: 3 },
    Instruction { name: "input", opcode: 3, func: op_in, number_parameters: 1 },
    Instruction { name: "output", opcode: 4, func: op_out, number_parameters: 1 },
    Instruction { name: "jump-if-true", opcode: 5, func: op_jit, number_parameters: 2 },
    Instruction { name: "jump-if-false", opcode: 6, func: op_jif, number_parameters: 2 },
    Instruction { name: "less than", opcode: 7, func: op_lt, number_parameters: 3 },
    Instruction { name: "equals", opcode: 8, func: op_eq, number_parameters: 3 },
    Instruction { name: "relative base offset", opcode: 9, func: op_relb, number_parameters: 1 },
];

pub struct Instruction {
    opcode: u8,
    name: &'static str,
    func: fn(&mut Interpreter) -> Option<InterpreterError>,
    number_parameters: usize,
}

fn op_halt(pc: &mut Interpreter) -> Option<InterpreterError> {
    pc.finish = true;
    pc.ip += 0;
    Some(InterpreterError::Terminated)
}

fn op_add(pc: &mut Interpreter) -> Option<InterpreterError> {
    if let Some(val) = pc.code[pc.param_indices[0] as usize].checked_add(pc.code[pc.param_indices[1] as usize]) {
        pc.code[pc.param_indices[2] as usize] = val;
        pc.ip += 4;
        None
    } else {
        Some(InterpreterError::Overflow)
    }
}

fn op_mul(pc: &mut Interpreter) -> Option<InterpreterError> {
    if let Some(val) = pc.code[pc.param_indices[0] as usize].checked_mul(pc.code[pc.param_indices[1] as usize]) {
        pc.code[pc.param_indices[2] as usize] = val;
        pc.ip += 4;
        None
    } else {
        Some(InterpreterError::Overflow)
    }
}

fn op_in(pc: &mut Interpreter) -> Option<InterpreterError> {
    if let Some(val) = pc.input_buffer.pop_front() {
        pc.code[pc.param_indices[0]] = val;
        pc.ip += 2;
        return None
    }

    Some(InterpreterError::NoInputError)
}

fn op_out(pc: &mut Interpreter) -> Option<InterpreterError> {
    let res = pc.code[pc.param_indices[0]];

    pc.last_output = Some(res);
    pc.ip += 2;
    None
}

fn op_jit(pc: &mut Interpreter) -> Option<InterpreterError> {
    if pc.code[pc.param_indices[0]] != 0 {
        if let Some(&val) = pc.code.get(pc.param_indices[1]) {
            pc.ip = val as usize;
            None
        } else {
            Some(InterpreterError::JumpOutOfBounds)
        }
    } else {
        pc.ip += 3;
        None
    }
}

fn op_jif(pc: &mut Interpreter) -> Option<InterpreterError> {
    if pc.code[pc.param_indices[0]] == 0 {
        if let Some(&val) = pc.code.get(pc.param_indices[1]) {
            pc.ip = val as usize;
            None
        } else {
            Some(InterpreterError::JumpOutOfBounds)
        }
    } else {
        pc.ip += 3;
        None
    }
}

fn op_lt(pc: &mut Interpreter) -> Option<InterpreterError> {
    pc.code[pc.param_indices[2]] = (pc.code[pc.param_indices[0]] < pc.code[pc.param_indices[1]]) as VALUE;
    pc.ip += 4;
    None
}

fn op_eq(pc: &mut Interpreter) -> Option<InterpreterError> {
    pc.code[pc.param_indices[2]] = (pc.code[pc.param_indices[0]] == pc.code[pc.param_indices[1]]) as VALUE;
    pc.ip += 4;
    None
}

fn op_relb(pc: &mut Interpreter) -> Option<InterpreterError> {
    pc.relative_base = (pc.relative_base as i64 + pc.code[pc.param_indices[0] as usize] as i64) as isize;
    pc.ip += 2;
    None
}

