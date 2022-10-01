use src::Interpreter;
use std::{io, collections::VecDeque};
use itertools::Itertools;

mod src;


const DAY2A_RESULT: src::VALUE = 6087827;
const DAY2B_RESULT: src::VALUE = 5379;
const DAY5A_RESULT: src::VALUE = 5182797;
const DAY5B_RESULT: src::VALUE = 12077198;
const DAY7A_RESULT: src::VALUE = 77500;
const DAY7B_RESULT: src::VALUE = 22476942;
const DAY9A_RESULT: src::VALUE = 2406950601;
const DAY9B_RESULT: src::VALUE = 83239;


fn main() {
    //let day2_test: Vec<i64> = vec![1,9,10,3,2,3,11,0,99,30,40,50];
    day9();
}

fn execute(code_str: &str, input_buffer: VecDeque<src::VALUE>) -> Result<Vec<src::VALUE>, src::InterpreterError> {
    let data: Vec<src::VALUE> = code_str
        .trim()
        .split(',')
        .map(|x| str::parse(x).unwrap())
        .collect();

    let mut pc = Interpreter::new(data.to_owned(), input_buffer);
    let mut output = vec![];

    loop {
        let res = pc.step_loop();

        match res {
            Err(src::InterpreterError::Terminated) => {break},
            Err(e) => { return Err(e); }, 
            Ok(Some(val)) => { output.push(val); },
            Ok(None) => {},
        }
    }

    Ok(output)
}


fn day9() -> () {
    let data9: Vec<src::VALUE> = include_str!("../data/day9.txt")
        .trim()
        .split(',')
        .map(|x| str::parse(x).unwrap())
        .collect();

        let mut pc = Interpreter::new(data9.to_owned(), vec![1].into());

        println!("{:?}", pc);
        loop {
            let res = pc.step_loop();

            if let Err(src::InterpreterError::Terminated) = res {
                break
            }

            if let Ok(Some(val)) = res {
                println!("Output: {}", val);
            }
        }

}

fn day7() -> src::VALUE {
    let data7: Vec<_> = include_str!("../data/day7.txt")
        .trim()
        .split(',')
        .map(|x| str::parse(x).unwrap())
        .collect();


    let mut max_phase_amplifier = 0;

    for inputs in (0..=4).permutations(5) {
        println!("{:?}", inputs);

        let mut a1 = Interpreter::new(data7.to_owned(), vec![inputs[0], 0].into());
        let res1 = a1.step_loop().unwrap().unwrap();

        let mut a2 = Interpreter::new(data7.to_owned(), vec![inputs[1], res1].into());
        let res2 = a2.step_loop().unwrap().unwrap();

        let mut a3 = Interpreter::new(data7.to_owned(), vec![inputs[2], res2].into());
        let res3 = a3.step_loop().unwrap().unwrap();

        let mut a4 = Interpreter::new(data7.to_owned(), vec![inputs[3], res3].into());
        let res4 = a4.step_loop().unwrap().unwrap();

        let mut a5 = Interpreter::new(data7.to_owned(), vec![inputs[4], res4].into());
        let res5 = a5.step_loop().unwrap().unwrap();

        max_phase_amplifier = max_phase_amplifier.max(res5);
    }

    println!("Highest signal strength: {}", max_phase_amplifier);
    max_phase_amplifier
}

fn day7b() -> src::VALUE {
    let data7: Vec<_> = include_str!("../data/day7.txt")
    //let data7: Vec<_> = "3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5"
        .trim()
        .split(',')
        .map(|x| str::parse(x).unwrap())
        .collect();

    let mut max_phase_amplifier = 0;
    let mut counter = 0;

    for inputs in (5..=9).permutations(5) {
        counter += 1;

        let mut a1 = Interpreter::new(data7.to_owned(), vec![inputs[0], 0].into());
        let mut a2 = Interpreter::new(data7.to_owned(), vec![inputs[1]].into());
        let mut a3 = Interpreter::new(data7.to_owned(), vec![inputs[2]].into());
        let mut a4 = Interpreter::new(data7.to_owned(), vec![inputs[3]].into());
        let mut a5 = Interpreter::new(data7.to_owned(), vec![inputs[4]].into());


        loop {
            let res1 = match a1.step_loop() {
                Ok(Some(res)) => { res },
                Err(src::InterpreterError::Terminated) => { 
                    a2.step_loop().unwrap_err();
                    a3.step_loop().unwrap_err();
                    a4.step_loop().unwrap_err();
                    max_phase_amplifier = max_phase_amplifier.max(a5.last_output.unwrap());
                    a5.step_loop().unwrap_err();
                    break
                },
                _ => { panic!() },
            };

            a2.input_buffer.push_back(res1);
            let res2 = a2.step_loop().unwrap().unwrap();
            a3.input_buffer.push_back(res2);
            let res3 = a3.step_loop().unwrap().unwrap();
            a4.input_buffer.push_back(res3);
            let res4 = a4.step_loop().unwrap().unwrap();
            a5.input_buffer.push_back(res4);
            let res5 = a5.step_loop().unwrap().unwrap();
            a1.input_buffer.push_back(res5);
        }
    }



    println!("Counter: {}", counter);
    println!("Highest signal strength: {}", max_phase_amplifier);
    max_phase_amplifier
}

/// Solution AoC2019/Day2. Intcode challenge: 1
fn day2() -> src::VALUE {
    let mut data2: Vec<_> = include_str!("../data/day2.txt")
        .trim()
        .split(',')
        .map(|x| str::parse(x).unwrap())
        .collect();

    data2[1] = 12;
    data2[2] = 2;

    //let mut pc: Interpreter = Interpreter { code: data2, ip: 0, param_indices: vec![], finish: false, output_stream: Box::new(io::stdout()) };
    let mut pc: Interpreter = Default::default();
    pc.code = data2;

    while !pc.finish {
        println!("{:?}", pc);
        pc.step();
    }

    println!("{}", pc.code[0]);
    return pc.code[0];
}

/// Solution AoC2019/Day2b. Intcode challenge: 2
fn day2b() -> src::VALUE {
    let data2: Vec<_> = include_str!("../data/day2.txt")
        .trim()
        .split(',')
        .map(|x| str::parse(x).unwrap())
        .collect();

    let target = 19690720;

    for noun in 0..=99 {
        for verb in 0..=99 {
            let mut data = data2.to_owned();
            data[1] = noun;
            data[2] = verb;

            let mut pc: Interpreter = Default::default();
            pc.code = data;

            while !pc.finish {
                pc.step();
            }

            if pc.code[0] == target {
                let res = 100*noun + verb;
                println!("The answer is {}", res);
                return res;
            }
        }
    }
    unreachable!()
}

/// Solution AoC2019/Day5. Intcode challenge: 3
fn day5() -> src::VALUE {
    let data5: Vec<_> = include_str!("../data/day5.txt")
        .trim()
        .split(',')
        .map(|x| str::parse(x).unwrap())
        .collect();

    //let mut pc: Interpreter = Interpreter { code: data5, ip: 0, param_indices: vec![], finish: false };
    let mut pc: Interpreter = Default::default();
    pc.code = data5;

    while !pc.finish {
        //println!("{:?}", pc);
        pc.step();
    }

    let res = pc.code[pc.ip - 1];
    println!("{}", res);
    res
    
}



/// Solution AoC2019/Day5. Intcode challenge: 4
fn day5b() -> src::VALUE {
    let data5: Vec<_> = include_str!("../data/day5.txt")
        .trim()
        .split(',')
        .map(|x| str::parse(x).unwrap())
        .collect();

    //let mut pc: Interpreter = Interpreter { code: data5, ip: 0, param_indices: vec![], finish: false };
    let mut pc: Interpreter = Default::default();
    pc.code = data5;

    while !pc.finish {
        //println!("{:?}", pc);
        pc.step();
    }

    let res = pc.code[pc.ip - 1];
    println!("{}", res);
    res
}

fn day5tests() {
    let test1 = vec![3,9,8,9,10,9,4,9,99,-1,8];
    //let data5: Vec<_> = vec![3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99];
    //let mut pc: Interpreter = Interpreter { code: test1, ip: 0, param_indices: vec![], finish: false };
    let mut pc: Interpreter = Default::default();
    pc.code = test1;

    while !pc.finish {
        //println!("{:?}", pc);
        pc.step();
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_first_example() {
        let code = vec![1,9,10,3,2,3,11,0,99,30,40,50];
        //let mut pc: Interpreter = Interpreter { code, ip: 0, param_indices: vec![], finish: false, output_stream: Box::new(io::stdout())  };
        let mut pc: Interpreter = Default::default();
        pc.code = code;

        while !pc.finish {
            //println!("{:?}", pc);
            pc.step();
        }
        let wanted = vec![3500,9,10,70, 2,3,11,0, 99, 30,40,50];

        assert_eq!(pc.code, wanted);
    }

    // Solution AoC2019/Day2. Intcode challenge: 1
    #[test]
    fn day2a() {
        let mut data2: Vec<_> = include_str!("../data/day2.txt")
            .trim()
            .split(',')
            .map(|x| str::parse(x).unwrap())
            .collect();

        data2[1] = 12;
        data2[2] = 2;

        let mut pc = Interpreter::new(data2, vec![].into());

        let res = pc.step_loop();

        if let Err(src::InterpreterError::Terminated) = res {
        } else {
            assert!(false);
        }

        assert_eq!(pc.code[0], DAY2A_RESULT);
    }

    // Solution AoC2019/Day2b. Intcode challenge: 2
    #[test]
    fn day2b() {
        let data2: Vec<_> = include_str!("../data/day2.txt")
            .trim()
            .split(',')
            .map(|x| str::parse(x).unwrap())
            .collect();

        let target = 19690720;

        for noun in 0..=99 {
            for verb in 0..=99 {
                let mut data = data2.to_owned();
                data[1] = noun;
                data[2] = verb;

                let mut pc = Interpreter::new(data, vec![].into());

                let res = pc.step_loop();

                if let Err(src::InterpreterError::Terminated) = res {
                } else {
                    assert!(false);
                }

                if pc.code[0] == target {
                    let result = 100*noun + verb;
                    assert_eq!(result, DAY2B_RESULT);
                    return
                }
            }
        }
        unreachable!()
    }


    #[test]
    fn day5a() {
        let mut wanted = vec![0; 10];
        wanted[9] = DAY5A_RESULT;
        let wanted = wanted;

        let given = execute(include_str!("../data/day5.txt"), vec![1].into()).unwrap();

        assert_eq!(wanted, given);
    }
    //#[test]
    //fn day5a() {
        //let data5: Vec<_> = include_str!("../data/day5.txt")
            //.trim()
            //.split(',')
            //.map(|x| str::parse(x).unwrap())
            //.collect();

        //let mut pc = Interpreter::new(data5, vec![1].into());
        //let mut output = vec![];

        //loop {
            //let res = pc.step_loop();
            //match res {
                //Err(src::InterpreterError::Terminated) => { break; },
                //Err(_) => { panic!() },
                //Ok(Some(val)) => { println!("Output: {}", val); output.push(val); },
                //Ok(None) => {},
            //}
        //}

        //let result = output.pop().unwrap();
        
        //assert!(output.into_iter().all(|x| x == 0));
        //assert_eq!(result, DAY5A_RESULT);
    //}

    #[test]
    fn day5b() {
        let data5: Vec<_> = include_str!("../data/day5.txt")
            .trim()
            .split(',')
            .map(|x| str::parse(x).unwrap())
            .collect();

        let mut pc = Interpreter::new(data5, vec![5].into());
        let mut output = vec![];

        loop {
            let res = pc.step_loop();
            match res {
                Err(src::InterpreterError::Terminated) => { break; },
                Err(_) => { panic!() },
                Ok(Some(val)) => { println!("Output: {}", val); output.push(val); },
                Ok(None) => {},
            }
        }

        let result = output.pop().unwrap();
        
        assert!(output.into_iter().all(|x| x == 0));
        assert_eq!(result, DAY5B_RESULT);
    }

    #[test]
    fn day7a() {
        let data7: Vec<_> = include_str!("../data/day7.txt")
            .trim()
            .split(',')
            .map(|x| str::parse(x).unwrap())
            .collect();

        let mut signal_strength = 0;

        for inputs in (0..=4).permutations(5) {
            let mut a1 = Interpreter::new(data7.to_owned(), vec![inputs[0], 0].into());
            let res1 = a1.step_loop().unwrap().unwrap();

            let mut a2 = Interpreter::new(data7.to_owned(), vec![inputs[1], res1].into());
            let res2 = a2.step_loop().unwrap().unwrap();

            let mut a3 = Interpreter::new(data7.to_owned(), vec![inputs[2], res2].into());
            let res3 = a3.step_loop().unwrap().unwrap();

            let mut a4 = Interpreter::new(data7.to_owned(), vec![inputs[3], res3].into());
            let res4 = a4.step_loop().unwrap().unwrap();

            let mut a5 = Interpreter::new(data7.to_owned(), vec![inputs[4], res4].into());
            let res5 = a5.step_loop().unwrap().unwrap();

            signal_strength = signal_strength.max(res5);
        }

        assert_eq!(signal_strength, DAY7A_RESULT);
    }

    #[test]
    fn day7b() {
        let data7: Vec<_> = include_str!("../data/day7.txt")
            .trim()
            .split(',')
            .map(|x| str::parse(x).unwrap())
            .collect();

        let mut signal_strength = 0;

        for inputs in (5..=9).permutations(5) {
            let mut a1 = Interpreter::new(data7.to_owned(), vec![inputs[0], 0].into());
            let mut a2 = Interpreter::new(data7.to_owned(), vec![inputs[1]].into());
            let mut a3 = Interpreter::new(data7.to_owned(), vec![inputs[2]].into());
            let mut a4 = Interpreter::new(data7.to_owned(), vec![inputs[3]].into());
            let mut a5 = Interpreter::new(data7.to_owned(), vec![inputs[4]].into());


            loop {
                let res1 = match a1.step_loop() {
                    Ok(Some(res)) => { res },
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
                let res2 = a2.step_loop().unwrap().unwrap();
                a3.input_buffer.push_back(res2);
                let res3 = a3.step_loop().unwrap().unwrap();
                a4.input_buffer.push_back(res3);
                let res4 = a4.step_loop().unwrap().unwrap();
                a5.input_buffer.push_back(res4);
                let res5 = a5.step_loop().unwrap().unwrap();
                a1.input_buffer.push_back(res5);
            }
        }
        assert_eq!(signal_strength, DAY7B_RESULT);
    }

    #[test]
    fn day9a() {
        let data9: Vec<src::VALUE> = include_str!("../data/day9.txt")
            .trim()
            .split(',')
            .map(|x| str::parse(x).unwrap())
            .collect();

        let mut pc = Interpreter::new(data9.to_owned(), vec![1].into());
        let mut output = vec![];

        loop {
            let res = pc.step_loop();

            match res {
                Err(src::InterpreterError::Terminated) => {break},
                Err(_) => {panic!()}, 
                Ok(Some(val)) => {println!("Output: {}", val); output.push(val);},
                Ok(None) => {},
            }
        }

        let res = output.pop().unwrap();

        assert!(output.iter().all(|&x| x == 0));
        assert_eq!(res, DAY9A_RESULT);
    }

    #[test]
    fn day9b() {
        let data9: Vec<src::VALUE> = include_str!("../data/day9.txt")
            .trim()
            .split(',')
            .map(|x| str::parse(x).unwrap())
            .collect();

        let mut pc = Interpreter::new(data9.to_owned(), vec![2].into());
        let mut output = vec![];

        loop {
            let res = pc.step_loop();

            match res {
                Err(src::InterpreterError::Terminated) => {break},
                Err(_) => {panic!()}, 
                Ok(Some(val)) => {println!("Output: {}", val); output.push(val);},
                Ok(None) => {},
            }
        }

        assert!(output.len() == 1);
        let res = output.pop().unwrap();
        assert_eq!(res, DAY9B_RESULT);
    }

}
