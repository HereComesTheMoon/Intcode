use src::Interpreter;
use std::io;

mod src;

fn main() {
    //let day2_test: Vec<i64> = vec![1,9,10,3,2,3,11,0,99,30,40,50];
    //check_read_write();
}


//fn check_read_write() {
    //// First, jump past memory registers which are stored at the start of the code
    //// prg ~ beginning of code, mem ~ beginning of memory registers
    //let prg = 10;
    //let mem = 3;
    //// Write content of mem0 to output thrice.
    ////let writer_code = vec![1105, 1, prg, -1, -1, -1, -1, -1, -1, -1,
                            ////04, mem, 04, mem, 04, mem, 99];
    ////let reader_code = vec![1105, 1, prg, -1, -1, -1, -1, -1, -1, -1,
                            ////04, mem, 04, mem, 04, mem, 99];

    //// Read content, then write content
    //let streamer_code = vec![1105, 1, prg, -1, -1, -1, -1, -1, -1, -1,
                            //03, mem, 04, mem, 1105, 1, prg];


    //let mut out1_buf: &'static Vec<u8> = &vec![];
    //let out1 = Box::new(io::BufWriter::new(&mut out1_buf));
    //let mut pc1: Interpreter = Interpreter::new(streamer_code.to_owned(), out1, Box::new(io::stdin().lock()));


    //println!("{:?}", pc1.code);
    //loop {
        //pc1.step();
        //println!("{:?}", pc1.code);
    //}
//}

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
        //println!("{:?}", pc);
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
    use std::io;

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

    #[test]
    fn check_day_2() {
        let res = day2();
        assert_eq!(res, 6087827);


        let res = day2b();
        assert_eq!(res, 5379);
    }

    //#[test]
    //fn check_day_5() {
        //let data5: Vec<_> = include_str!("../data/day5.txt")
            //.trim()
            //.split(',')
            //.map(|x| str::parse(x).unwrap())
            //.collect();

        //let mut output = vec![];

        //{
            //let mut pc: Interpreter = Default::default();
            //pc.code = data5;

            //pc.input_stream = Box::new(io::BufReader::new(b"1" as &[u8]));

            //pc.output_stream = Box::new(io::BufWriter::new(&mut output));

            //while !pc.finish {
                //pc.step();
            //}

            //(*pc.output_stream).flush().unwrap();
        //}       

        //let ss = String::from_utf8_lossy(&output);
        //let given: src::VALUE = ss
            //.trim()
            //.split_ascii_whitespace()
            //.last()
            //.unwrap()
            //.parse()
            //.unwrap();

        //assert_eq!(given, 5182797);
    //}

    //#[test]
    //fn check_day_5b() {
        //let data5: Vec<_> = include_str!("../data/day5.txt")
            //.trim()
            //.split(',')
            //.map(|x| str::parse(x).unwrap())
            //.collect();

        //let mut output = vec![];

        //{
            //let mut pc: Interpreter = Default::default();
            //pc.code = data5;

            //pc.input_stream = Box::new(io::BufReader::new(b"5" as &[u8]));

            //pc.output_stream = Box::new(io::BufWriter::new(&mut output));

            //while !pc.finish {
                //pc.step();
            //}

            //(*pc.output_stream).flush().unwrap();
        //}       

        //let ss = String::from_utf8_lossy(&output);
        //let given: src::VALUE = ss
            //.trim()
            //.split_ascii_whitespace()
            //.last()
            //.unwrap()
            //.parse()
            //.unwrap();

        //assert_eq!(given, 12077198);
    //}

    #[test]
    fn check_read_write() {
        let a = 13;
        //let reader_code = vec![103, a, 104, a, 103, a, 104, a, 103, a, 104, a, 99, 0];
        let writer_code = vec![104, a, 104, a, 104, a, 104, a, 104, a, 104, a, 99, 12345];

        let mut pc: Interpreter = Default::default();
        pc.code = writer_code;
        while !pc.finish {
            pc.step();
        }
    }
}
