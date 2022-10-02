use src::Interpreter;
use std::collections::VecDeque;
use std::collections::HashMap;
use num_complex::Complex;

mod src;

fn main() {
    execute(vec![99], vec![].into());

    day13a()

}

fn day13a() {
}

fn day11a() {
    let data = string_to_code(include_str!("../data/day11.txt"));

    let mut pos: Complex<i64> = Complex::new(0, 0);
    let mut dir: Complex<i64> = Complex::new(-1, 0);
    let mut pc = Interpreter::new(data, vec![0].into());

    let mut tiles: HashMap<Complex<i64>, bool> = HashMap::new();

    loop {
        let first = pc.step_loop();
        match first {
            Err(src::InterpreterError::Terminated) => { break; },
            Err(_) => { panic!() },
            Ok(color) => { tiles.insert(pos, color != 0) },
        };

        // Only works as long as turn_direction is in {0, 1}
        let second = pc.step_loop();
        match second {
            Err(src::InterpreterError::Terminated) => { break; },
            Err(_) => { panic!() },
            Ok(turn_direction) => { dir = dir * Complex::new(0, 1 - 2*turn_direction)},
        };

        pos += dir;
        pc.input_buffer.push_back(*tiles.get(&pos).unwrap_or(&false) as i64);
    }

    println!("Number of tiles painted at least once: {}", tiles.len());
    // 2418, correct!
}

fn day11b() {
    let data = string_to_code(include_str!("../data/day11.txt"));

    let mut pos: Complex<i64> = Complex::new(0, 0);
    // Starting direction. Starting with -1, 0 results in grid extending in the nicest direction
    let mut dir: Complex<i64> = Complex::new(-1, 0);
    let mut pc = Interpreter::new(data, vec![1].into());

    let mut tiles: HashMap<Complex<i64>, bool> = HashMap::new();

    loop {
        let first = pc.step_loop();
        match first {
            Err(src::InterpreterError::Terminated) => { break; },
            Err(_) => { panic!() },
            Ok(color) => { tiles.insert(pos, color != 0) },
        };

        // Only works as long as turn_direction is in {0, 1}
        let second = pc.step_loop();
        match second {
            Err(src::InterpreterError::Terminated) => { break; },
            Err(_) => { panic!() },
            Ok(turn_direction) => { dir = dir * Complex::new(0, 1 - 2*turn_direction)},
        };

        pos += dir;
        pc.input_buffer.push_back(*tiles.get(&pos).unwrap_or(&false) as i64);
    }

    let minx = tiles.keys().map(|&x| x.re as i32).min().unwrap();
    let miny = tiles.keys().map(|&x| x.im as i32).min().unwrap();
    let maxx = tiles.keys().map(|&x| x.re as i32).max().unwrap();
    let maxy = tiles.keys().map(|&x| x.im as i32).max().unwrap();

    let mut grid: Vec<Vec<char>> = vec![vec!['.'; (miny.abs()+maxy+1) as usize]; (minx.abs()+maxx+1) as usize];

    for (z, &color) in tiles.iter() {
        let paint = if color { '#' } else { '.' };
        grid[z.re as usize][z.im as usize] = paint;
    }

    println!("{}, {}", grid.len(), grid[0].len());
    for row in grid {
        for x in row {
            print!("{}", x);
        }
        println!();
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
    const DAY2A_RESULT: src::VALUE = 6087827;
    const DAY2B_RESULT: src::VALUE = 5379;
    const DAY5A_RESULT: src::VALUE = 5182797;
    const DAY5B_RESULT: src::VALUE = 12077198;
    const DAY7A_RESULT: src::VALUE = 77500;
    const DAY7B_RESULT: src::VALUE = 22476942;
    const DAY9A_RESULT: src::VALUE = 2406950601;
    const DAY9B_RESULT: src::VALUE = 83239;
    use itertools::Itertools;

        

    // Solution AoC2019/Day2. Intcode challenge: 1
    // First AoC Intcode challenge. Requires Interpreter.code to be public, hence doesn't compile now. Otherwise passes.
    //#[test]
    //fn day2a() {
        //let mut data2: Vec<_> = string_to_code(include_str!("../data/day2.txt"));

        //data2[1] = 12;
        //data2[2] = 2;

        //let mut pc = Interpreter::new(data2, vec![].into());

        //let res = pc.step_loop();

        //if let Err(src::InterpreterError::Terminated) = res {
        //} else {
            //assert!(false);
        //}

        //assert_eq!(pc.code[0], DAY2A_RESULT);
    //}

    // Solution AoC2019/Day2b. Intcode challenge: 2
    // Second AoC Intcode challenge. Requires Interpreter.code to be public, hence doesn't compile now. Otherwise passes.
    //#[test]
    //fn day2b() {
        //let data2: Vec<_> = string_to_code(include_str!("../data/day2.txt"));

        //let target = 19690720;

        //for noun in 0..=99 {
            //for verb in 0..=99 {
                //let mut data = data2.to_owned();
                //data[1] = noun;
                //data[2] = verb;

                //let mut pc = Interpreter::new(data, vec![].into());

                //let res = pc.step_loop();

                //if let Err(src::InterpreterError::Terminated) = res {
                //} else {
                    //assert!(false);
                //}

                //if pc.code[0] == target {
                    //let result = 100*noun + verb;
                    //assert_eq!(result, DAY2B_RESULT);
                    //return
                //}
            //}
        //}
        //unreachable!()
    //}


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
        let wanted = vec![DAY5B_RESULT];

        let given = execute(string_to_code(include_str!("../data/day5.txt")), vec![5].into()).unwrap();

        assert_eq!(wanted, given);
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
        let data7: Vec<_> = string_to_code(include_str!("../data/day7.txt"));

        let mut signal_strength = 0;

        for inputs in (0..=4).permutations(5) {
            let mut a1 = Interpreter::new(data7.to_owned(), vec![inputs[0], 0].into());
            let res1 = a1.step_loop().unwrap();

            let mut a2 = Interpreter::new(data7.to_owned(), vec![inputs[1], res1].into());
            let res2 = a2.step_loop().unwrap();

            let mut a3 = Interpreter::new(data7.to_owned(), vec![inputs[2], res2].into());
            let res3 = a3.step_loop().unwrap();

            let mut a4 = Interpreter::new(data7.to_owned(), vec![inputs[3], res3].into());
            let res4 = a4.step_loop().unwrap();

            let mut a5 = Interpreter::new(data7.to_owned(), vec![inputs[4], res4].into());
            let res5 = a5.step_loop().unwrap();

            signal_strength = signal_strength.max(res5);
        }

        assert_eq!(signal_strength, DAY7A_RESULT);
    }

    #[test]
    fn day7b() {
        let data7: Vec<_> = string_to_code(include_str!("../data/day7.txt"));

        let mut signal_strength = 0;

        for inputs in (5..=9).permutations(5) {
            let mut a1 = Interpreter::new(data7.to_owned(), vec![inputs[0], 0].into());
            let mut a2 = Interpreter::new(data7.to_owned(), vec![inputs[1]].into());
            let mut a3 = Interpreter::new(data7.to_owned(), vec![inputs[2]].into());
            let mut a4 = Interpreter::new(data7.to_owned(), vec![inputs[3]].into());
            let mut a5 = Interpreter::new(data7.to_owned(), vec![inputs[4]].into());


            loop {
                let res1 = match a1.step_loop() {
                    Ok(res) => { res },
                    Err(src::InterpreterError::Terminated) => { 
                        // Once the first amplifier terminates, all the amplifiers will terminate
                        a2.step_loop().unwrap_err();
                        a3.step_loop().unwrap_err();
                        a4.step_loop().unwrap_err();
                        signal_strength = signal_strength.max(a5.last_output.unwrap());
                        a5.step_loop().unwrap_err();
                        break
                    },
                    _ => { panic!() },
                };

                a2.input_buffer.push_back(res1);
                let res2 = a2.step_loop().unwrap();
                a3.input_buffer.push_back(res2);
                let res3 = a3.step_loop().unwrap();
                a4.input_buffer.push_back(res3);
                let res4 = a4.step_loop().unwrap();
                a5.input_buffer.push_back(res4);
                let res5 = a5.step_loop().unwrap();
                a1.input_buffer.push_back(res5);
            }
        }
        assert_eq!(signal_strength, DAY7B_RESULT);
    }

    #[test]
    fn day9a() {
        let wanted = vec![DAY9A_RESULT];

        let given = execute(string_to_code(include_str!("../data/day9.txt")), vec![1].into()).unwrap();

        assert_eq!(wanted, given);
    }

    #[test]
    #[ignore = "This is more of a benchmark. Disabled to cut down on waiting time."]
    fn day9b() {
        let wanted = vec![DAY9B_RESULT];

        let given = execute(string_to_code(include_str!("../data/day9.txt")), vec![2].into()).unwrap();

        assert_eq!(wanted, given);
    }

}
