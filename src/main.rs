use src::Interpreter;

mod src;

fn main() {
    //let day2_test: Vec<i64> = vec![1,9,10,3,2,3,11,0,99,30,40,50];
    day5b();
}


/// Solution AoC2019/Day2. Intcode challenge: 1
fn day2() {
    let mut data2: Vec<_> = include_str!("../data/day2.txt")
        .trim()
        .split(',')
        .map(|x| str::parse(x).unwrap())
        .collect();

    data2[1] = 12;
    data2[2] = 2;

    let mut pc: Interpreter = Interpreter { code: data2, ip: 0, param_indices: vec![], finish: false };

    while !pc.finish {
        println!("{:?}", pc);
        pc.step();
    }

    println!("{}", pc.code[0]);
}

/// Solution AoC2019/Day2b. Intcode challenge: 2
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

            let mut pc: Interpreter = Interpreter { code: data, ip: 0, param_indices: vec![], finish: false };

            while !pc.finish {
                pc.step();
            }

            if pc.code[0] == target {
                let res = 100*noun + verb;
                println!("The answer is {}", res);
                return 
            }
        }
    }
    unreachable!()
}

/// Solution AoC2019/Day5. Intcode challenge: 3
fn day5() {
    let data5: Vec<_> = include_str!("../data/day5.txt")
        .trim()
        .split(',')
        .map(|x| str::parse(x).unwrap())
        .collect();

    let mut pc: Interpreter = Interpreter { code: data5, ip: 0, param_indices: vec![], finish: false };

    while !pc.finish {
        println!("{:?}", pc);
        pc.step();
    }

    println!("{}", pc.code[pc.ip - 1]);
}



/// Solution AoC2019/Day5. Intcode challenge: 4
fn day5b() {
    let data5: Vec<_> = include_str!("../data/day5.txt")
        .trim()
        .split(',')
        .map(|x| str::parse(x).unwrap())
        .collect();

    let mut pc: Interpreter = Interpreter { code: data5, ip: 0, param_indices: vec![], finish: false };

    while !pc.finish {
        println!("{:?}", pc);
        pc.step();
    }

    println!("{}", pc.code[pc.ip - 1]);
}

fn day5tests() {
    let test1 = vec![3,9,8,9,10,9,4,9,99,-1,8];
    //let data5: Vec<_> = vec![3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99];
    let mut pc: Interpreter = Interpreter { code: test1, ip: 0, param_indices: vec![], finish: false };

    while !pc.finish {
        println!("{:?}", pc);
        pc.step();
    }
}


