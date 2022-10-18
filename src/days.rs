use src::Interpreter;
use std::collections::VecDeque;
use std::collections::HashMap;
use std::fmt::Debug;
use std::fmt::Display;
use num_complex::Complex;

use crate::src;

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


pub mod day17 {
    use super::*;
    pub fn camera() {
        let data = string_to_code(include_str!("../data/day17.txt"));
        let mut pc = Interpreter::new(data, vec![].into());

        loop {
            let c = pc.step_loop();

            match c {
                Err(src::InterpreterError::Terminated) => { break; },
                Err(e) => println!("Error: {:?}", e),
                Ok(c) => print!("{}", c as u8 as char),
            }
        }
    }
}

pub mod day15 {
    use super::*;

    const SIZE: (usize, usize) = (41, 41);
    const CENTER: (usize, usize) = (21, 21);

    #[derive(Clone, Copy)]
    pub enum Dir {
        N = 1,
        S = 2,
        W = 3,
        E = 4,
    }

    #[derive(Clone, Copy, PartialEq)]
    pub enum Tile {
        Empty,
        Wall,
        Goal,
        Unknown,
    }

    pub struct Game {
        pc: src::Interpreter,
        pub grid: [[(Tile, usize); SIZE.0]; SIZE.1],
        pos: (usize, usize),
    }

    impl Game {
        pub fn new() -> Game {
            let data = string_to_code(include_str!("../data/day15.txt"));
            let pc = Interpreter::new(data, vec![].into());

            Game {
                pc,
                grid: [[(Tile::Unknown, 0usize); SIZE.0]; SIZE.1],
                pos: (CENTER.0, CENTER.1),
            }
        }


        fn dfs(&mut self) -> Option<usize> {
            println!("{}", self);
            for d in [Dir::N, Dir::S, Dir::W, Dir::E] {
                let prev = self.pos;

                let next = match d {
                    Dir::N => {(prev.0 + 0, prev.1 - 1)},
                    Dir::S => {(prev.0 + 0, prev.1 + 1)},
                    Dir::W => {(prev.0 - 1, prev.1 + 0)},
                    Dir::E => {(prev.0 + 1, prev.1 + 0)},
                };


                if let Tile::Unknown = self.grid[next.1][next.0].0 {
                    self.pc.input_buffer.push_back(d as i64);

                    let val_prev = self.grid[prev.1][prev.0].1;
                    let res = match self.pc.step_loop() {
                        Err(_) => { panic!() },
                        Ok(0) => { Tile::Wall },
                        Ok(1) => { Tile::Empty },
                        Ok(2) => { Tile::Goal },
                        Ok(_) => { panic!() }
                    };

                    self.grid[next.1][next.0] = (res, val_prev + 1);

                    if res == Tile::Wall {
                        continue;
                    }

                    self.pos = next;
                    if res == Tile::Goal {
                        return Some(val_prev + 1);
                    }

                    if let Some(distance) = self.dfs() {
                        return Some(distance);
                    }

                    self.pc.input_buffer.push_back(
                        match d {
                            Dir::N => Dir::S as i64,
                            Dir::S => Dir::N as i64,
                            Dir::W => Dir::E as i64,
                            Dir::E => Dir::W as i64,
                        });

                    self.pc.step_loop().unwrap();
                    self.pos = prev;
                }
            }
            None
        }

        fn dfs_b(&mut self) -> usize {
            println!("{}", self);
            let mut maxi = 0;
            for d in [Dir::N, Dir::S, Dir::W, Dir::E] {
                let prev = self.pos;

                let next = match d {
                    Dir::N => {(prev.0 + 0, prev.1 - 1)},
                    Dir::S => {(prev.0 + 0, prev.1 + 1)},
                    Dir::W => {(prev.0 - 1, prev.1 + 0)},
                    Dir::E => {(prev.0 + 1, prev.1 + 0)},
                };


                if let Tile::Unknown = self.grid[next.1][next.0].0 {
                    self.pc.input_buffer.push_back(d as i64);

                    let val_prev = self.grid[prev.1][prev.0].1;
                    let res = match self.pc.step_loop() {
                        Err(_) => { panic!() },
                        Ok(0) => { Tile::Wall },
                        Ok(1) => { Tile::Empty },
                        Ok(2) => { Tile::Goal },
                        Ok(_) => { panic!() }
                    };

                    self.grid[next.1][next.0] = (res, val_prev + 1);

                    if res == Tile::Wall {
                        continue;
                    }

                    self.pos = next;

                    maxi = maxi.max(1 + self.dfs_b());


                    self.pc.input_buffer.push_back(
                        match d {
                            Dir::N => Dir::S as i64,
                            Dir::S => Dir::N as i64,
                            Dir::W => Dir::E as i64,
                            Dir::E => Dir::W as i64,
                        });

                    self.pc.step_loop().unwrap();
                    self.pos = prev;
                }
            }
            maxi
        }

    }

    pub fn day15a() -> src::VALUE {
        let mut game = Game::new();
        let dist = game.dfs().unwrap();
        println!("Distance to oxygen generator: {}", dist);
        dist as src::VALUE
    }

    pub fn day15b() -> src::VALUE {
        let mut game = Game::new();
        let dist = game.dfs().unwrap();
        println!("Distance to oxygen generator: {}", dist);

        // Reset game:
        game.grid = [[(Tile::Unknown, 0usize); SIZE.0]; SIZE.1];
        let dist = game.dfs_b();
        println!("Maximal distance from oxygen generator: {}", dist);
        dist as src::VALUE
    }


    impl Display for Game {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {

            let ss: String = self.grid
                .iter()
                .map(|&row| row
                     .iter()
                     .map(|&x| x.0
                          .to_string())
                     .collect::<String>() + "\n")
                .collect();

            write!(f, "{}", ss)
        }
    }

    impl Display for Tile {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", 
                   match *self {
                       Tile::Empty => " ",
                       Tile::Wall => "#",
                       Tile::Unknown => ".",
                       Tile::Goal => "X",
                   })
        }
    }

}

pub mod day13 {
    use std::{process::exit, time::Duration};

    use super::*;

    const SIZE_X: usize = 46;
    const SIZE_Y: usize = 26;

    pub struct Game {
        pc: src::Interpreter,
        score: i64,
        grid: [[Tile; SIZE_X]; SIZE_Y],
        number_blocks: usize,
        pos_ball: (usize, usize),
        pos_paddle: (usize, usize),
    }

    impl Game {
        pub fn new() -> Game {
            let mut data = string_to_code(include_str!("../data/day13.txt"));
            data[0] = 2;
            let mut pc = Interpreter::new(data, vec![].into());

            let mut grid = [[Tile::Empty; 46]; 26];

            loop {
                let posx = match pc.step_loop() {
                    //Err(src::InterpreterError::NoInputError) => { break; },
                    Err(_) => { panic!() },
                    Ok(val) => { val },
                };
                let posy = pc.step_loop().unwrap();
                let tile_tyle: Tile = match pc.step_loop().unwrap() {
                    0 => { Tile::Empty },
                    1 => { Tile::Wall },
                    2 => { Tile::Block },
                    3 => { Tile::Paddle },
                    4 => { Tile::Ball },
                    _ => { panic!() },
                };

                if (posx, posy) == (-1, 0) { break; }

                grid[posy as usize][posx as usize] = tile_tyle;
            }

            let number_blocks = grid
                .iter()
                .flatten()
                .filter(|&x| *x == Tile::Block )
                .count();

            Game {
                pc,
                score: 0,
                grid,
                number_blocks,
                pos_ball: (0,0),
                pos_paddle: (0,0),
            }
        }

        fn solve(&mut self) {
            loop {
                let res = self.pc.step_loop();
                //std::thread::sleep(Duration::new(0, 1_000_000));

                let posx = match res {
                    Err(src::InterpreterError::NoInputError) => { 
                        //let mut input: String = String::new();
                        assert!(self.pc.input_buffer.is_empty());
                        print!("{}", self);
                        print!("Ball: {:?}, Paddle: {:?}", self.pos_ball, self.pos_paddle);
                        //io::stdin().read_line(&mut input).unwrap();
                        
                        let next_in = if self.pos_paddle.0 > self.pos_ball.0 {
                            -1
                        } else if self.pos_paddle == self.pos_ball {
                            0
                        } else {
                            1
                        };
                        //let nextin = input.trim().parse().unwrap_or(next_in);

                        self.pc.input_buffer.push_back(next_in);
                        continue;
                    }
                    Err(src::InterpreterError::Terminated) => { println!("FINAL SCORE: {}", self.score); exit(0) }
                    Err(_) => {panic!()}
                    Ok(val) => { val }
                };
                let posy = self.pc.step_loop().unwrap();
                let val = self.pc.step_loop().unwrap();

                if (posx, posy) == (-1, 0) { 
                    self.score = val;
                } else {
                    let val = match val {
                        0 => { Tile::Empty },
                        1 => { Tile::Wall },
                        2 => { Tile::Block },
                        3 => { Tile::Paddle },
                        4 => { Tile::Ball },
                        _ => { panic!() },
                    };
                    if self.grid[posy as usize][posx as usize] == Tile::Block {
                        self.number_blocks -= 1;
                    }
                    if val == Tile::Ball {
                        self.pos_ball = (posx as usize, posy as usize);
                    }
                    if val == Tile::Paddle {
                        self.pos_paddle = (posx as usize, posy as usize);
                    }
                    self.grid[posy as usize][posx as usize] = val;
                }
            }
        }
    }

    impl Display for Game {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {

            let mut s: String = format!("{:-^SIZE_X$}\n", format!("Score: {}, Blocks left: {}", self.score, self.number_blocks));

            let ss: String = self.grid
                .iter()
                .map(|&row| row
                     .iter()
                     .map(|&x| x
                          .to_string())
                     .collect::<String>() + "\n")
                .collect();

            s.extend(ss.chars());

            write!(f, "{}", s)
        }
    }

    #[derive(Clone, Copy, PartialEq)]
    enum Tile {
        Empty,
        Wall,
        Block,
        Paddle,
        Ball,
    }

    impl Display for Tile {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", 
                   match *self {
                       Tile::Empty => ".",
                       Tile::Wall => "#",
                       Tile::Block => "X",
                       Tile::Paddle => "—",
                       Tile::Ball => "O",
                   })
        }
    }

    impl Debug for Tile {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", 
                   match *self {
                       Tile::Empty => ".",
                       Tile::Wall => "#",
                       Tile::Block => "X",
                       Tile::Paddle => "—",
                       Tile::Ball => "O",
                   })
        }
    }

    pub fn day13a() -> src::VALUE {
        let game = Game::new();
        print!("{}", game);
        return game.number_blocks as src::VALUE;
    }

    pub fn day13b() -> src::VALUE {
        let mut game = Game::new();
        game.solve();
        game.score
    }


}

pub mod day11 {
    use super::*;
    pub fn day11a() -> src::VALUE {
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
        tiles.len() as src::VALUE
        // 2418, correct!
    }

    pub fn day11b() -> src::VALUE {
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
        -1
    }
}

pub mod day9 {
    use super::*;

    pub fn day9a() -> src::VALUE {
        let result = execute(string_to_code(include_str!("../data/day9.txt")), vec![1].into()).unwrap()[0];

        println!("Result / BOOST Keycode: {}", result);
        result
    }

    pub fn day9b() -> src::VALUE {
        let result = execute(string_to_code(include_str!("../data/day9.txt")), vec![2].into()).unwrap()[0];

        println!("Result / Coordinates of distress signal: {}", result);
        result
    }
}

pub mod day7 {
    use super::*;
    use itertools::Itertools;

    pub fn day7a() -> src::VALUE {
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

        println!("Result / Highest signal: {}", signal_strength);
        signal_strength
    }

    pub fn day7b() -> src::VALUE {
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
        println!("Result / Highest signal: {}", signal_strength);
        signal_strength
    }
}

pub mod day5 {
    use super::*;

    pub fn day5a() -> src::VALUE {
        let result = execute(string_to_code(include_str!("../data/day5.txt")), vec![1].into()).unwrap()[9];

        println!("Result: {}", result);
        result
    }

    pub fn day5b() -> src::VALUE {
        let result = execute(string_to_code(include_str!("../data/day5.txt")), vec![5].into()).unwrap()[0];

        println!("Result: {}", result);
        result
    }
}

pub mod day2 {
    //use super::*;

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

}
