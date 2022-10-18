# INTCODE-Interpreter

See [here](https://esolangs.org/wiki/Intcode) for a quick overview of what Intcode is.

See Advent of Code 2019, [Day 2](https://adventofcode.com/2019/day/2), [Day 5](https://adventofcode.com/2019/day/5), [Day 7](https://adventofcode.com/2019/day/7), [Day 9](https://adventofcode.com/2019/day/9) for the Intcode-specification, and many example programs.

A new interpreter can easily be created:
```rust
let code = vec![4,17,4,19,1001,17,1,17,8,17,18,16,1006,16,0,99,-1,1,11,32];
let pc = src::Interpreter::new(code.to_owned(), vec![].into());
```

Executing `pc.step()` steps through the instructions one by one, `pc.step_loop()` steps through the instructions until either of the following happens:
1. There is an output value.
2. The program is requesting input, but the input buffer is empty.
3. The program terminated.
4. The program encountered a syntax error while parsing, eg. a jump out of memory, or an incorrect opcode.

These results can easily be handled with a simple match statement. For example:

```rust
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
}
```
