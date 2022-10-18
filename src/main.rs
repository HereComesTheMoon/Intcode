use src::Interpreter;
use std::collections::VecDeque;

mod src;

pub mod days;

fn main() {
    let code = vec![3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99];

    let mut pc = src::Interpreter::new(code.to_owned(), vec![].into());

    loop {
        let res = pc.step_loop();
        match res {
            Ok(val) => println!("{}", val),
            Err(src::InterpreterError::NoInputError) => { pc.input_buffer.push_back(read_input()) },
            Err(src::InterpreterError::Terminated) => { break; },
            Err(e) => {
                println!("Error! {:?}", e);
                println!("{:?}", pc);
                break;
            }
        }
    }

    //days::day17::camera();
    
    //days::day15::day15b();
    //days::day15::day15a();

    //days::day11::day11a();
    //days::day11::day11b();

    //days::day13::day13a();
    //days::day13::day13b();

    //days::day9::day9a();
    //days::day9::day9b();

    //days::day7::day7a();
    //days::day7::day7b();

    //days::day5::day5a();
    //days::day5::day5b();
}

fn read_input() -> src::VALUE {
    loop {
        println!("Input buffer empty. Use stdin. Waiting for input: ");

        let mut input = String::new();
        if let Err(e) = std::io::stdin().read_line(&mut input) {
            println!("Error: Interpreter failed to read input: {}", e);
            panic!()
        } 

        if let Ok(num) = input.trim().parse::<src::VALUE>() {
            return num
        }
    }
}

fn execute(data: Vec<src::VALUE>, input_buffer: VecDeque<src::VALUE>) -> Result<Vec<src::VALUE>, src::InterpreterError> {
    let mut pc = Interpreter::new(data.to_owned(), input_buffer);
    let mut output = vec![];

    loop {
        let res = pc.step_loop();

        match res {
            Err(src::InterpreterError::Terminated) => {break},
            Err(e) => { return Err(e); }, 
            Ok(val) => { output.push(val); },
        }
    }

    Ok(output)
}

fn string_to_code(code_str: &str) -> Vec<src::VALUE> {
    code_str
        .trim()
        .split(',')
        .map(|x| str::parse(x).unwrap())
        .collect()
}


#[cfg(test)]
mod tests {
    use super::*;
    use src::{Interpreter, InterpreterError};

    //const DAY2A_RESULT: src::VALUE = 6087827;
    //const DAY2B_RESULT: src::VALUE = 5379;
    const DAY5A_RESULT: src::VALUE = 5182797;
    const DAY5B_RESULT: src::VALUE = 12077198;
    const DAY7A_RESULT: src::VALUE = 77500;
    const DAY7B_RESULT: src::VALUE = 22476942;
    const DAY9A_RESULT: src::VALUE = 2406950601;
    const DAY9B_RESULT: src::VALUE = 83239;

    #[test]
    fn day5a() {
        let mut wanted = vec![0; 10];
        wanted[9] = DAY5A_RESULT;
        let wanted = wanted;

        let given = execute(string_to_code(include_str!("../data/day5.txt")), vec![1].into()).unwrap();

        assert_eq!(wanted, given);
    }

    #[test]
    fn day5b() {
        assert_eq!(DAY5B_RESULT, days::day5::day5b());
    }

    #[test]
    fn day5misc() {
        // Misc official tests. 
        let position_equal_code = string_to_code("3,9,8,9,10,9,4,9,99,-1,8");
        let position_smaller_code = string_to_code("3,9,7,9,10,9,4,9,99,-1,8");
        let immediate_equal_code = string_to_code("3,3,1108,-1,8,3,4,3,99");
        let immediate_smaller_code = string_to_code("3,3,1107,-1,8,3,4,3,99");


        let given = execute(position_equal_code.to_owned(), vec![8].into()).unwrap();
        assert_eq!(given, vec![1]);
        let given = execute(position_equal_code, vec![0].into()).unwrap();
        assert_eq!(given, vec![0]);

        let given = execute(position_smaller_code.to_owned(), vec![7].into()).unwrap();
        assert_eq!(given, vec![1]);
        let given = execute(position_smaller_code, vec![8].into()).unwrap();
        assert_eq!(given, vec![0]);

        let given = execute(immediate_equal_code.to_owned(), vec![8].into()).unwrap();
        assert_eq!(given, vec![1]);
        let given = execute(immediate_equal_code, vec![0].into()).unwrap();
        assert_eq!(given, vec![0]);

        let given = execute(immediate_smaller_code.to_owned(), vec![7].into()).unwrap();
        assert_eq!(given, vec![1]);
        let given = execute(immediate_smaller_code, vec![8].into()).unwrap();
        assert_eq!(given, vec![0]);


        let position_jump = string_to_code("3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9");
        let immediate_jump = string_to_code("3,3,1105,-1,9,1101,0,0,12,4,12,99,1");

        let given = execute(position_jump.to_owned(), vec![1].into()).unwrap();
        assert_eq!(given, vec![1]);
        let given = execute(position_jump, vec![0].into()).unwrap();
        assert_eq!(given, vec![0]);


        let given = execute(immediate_jump.to_owned(), vec![1].into()).unwrap();
        assert_eq!(given, vec![1]);
        let given = execute(immediate_jump, vec![0].into()).unwrap();
        assert_eq!(given, vec![0]);

        let larger_example = string_to_code("3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99");

        let given = execute(larger_example.to_owned(), vec![7].into()).unwrap();
        assert_eq!(given, vec![999]);
        let given = execute(larger_example.to_owned(), vec![8].into()).unwrap();
        assert_eq!(given, vec![1000]);
        let given = execute(larger_example.to_owned(), vec![9].into()).unwrap();
        assert_eq!(given, vec![1001]);

    }

    #[test]
    fn day7a() {
        assert_eq!(days::day7::day7a(), DAY7A_RESULT);
    }

    #[test]
    fn day7b() {
        assert_eq!(days::day7::day7b(), DAY7B_RESULT);
    }

    #[test]
    fn day9a() {
        let result = days::day9::day9a();
        assert_eq!(DAY9A_RESULT, result);
    }

    #[test]
    #[ignore = "This is more of a benchmark. Disabled to cut down on waiting time."]
    fn day9b() {
        assert_eq!(DAY9B_RESULT, days::day9::day9b());
    }

    #[test]
    fn wrong_code() {
        let mut pc = Interpreter::new(vec![-1, 0, 99], [].into());

        let e = pc.step();
        assert_eq!(e, Err(InterpreterError::InvalidOpCode));

        for op in [11301, 13101, 31101, 1_11101, 700001, 10001] {
            println!("{}", op);
            let mut pc = Interpreter::new(vec![op, 0, 99], [].into());
            let e = pc.step();
            assert_eq!(e, Err(InterpreterError::InvalidParameters));
        }

        let mut pc = Interpreter::new(vec![1101, 99, 99, 99], [].into());
        let e = pc.step();
        assert_eq!(e, Err(InterpreterError::InvalidParameters));
    }

    #[test]
    fn errors() {
        let mut pc = Interpreter::new(vec![3, 0, 99], [].into());

        let err = pc.step().unwrap_err();

        assert_eq!(err, InterpreterError::NoInputError);

        pc.input_buffer.push_back(-1);

        let nothing = pc.step().unwrap();

        assert_eq!(nothing, None);

        let err = pc.step().unwrap_err();

        assert_eq!(err, InterpreterError::Terminated);
    }


}
