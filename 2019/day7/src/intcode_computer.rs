pub struct Machine {
    tape: Vec<isize>,
    pos: usize,
    state: MachineState,
    outputs: Vec<isize>,
    inputs: Vec<isize>,
}

#[derive(Eq, PartialEq)]
enum MachineState {
    Cont,
    Halt,
    Error,
    Waiting,
}

impl Machine {
    pub fn new(tape: Vec<isize>) -> Machine {
        Machine {
            tape,
            pos: 0,
            state: MachineState::Cont,
            outputs: Vec::new(),
            inputs: Vec::new(),
        }
    }

    pub fn from(tape: &str) -> Machine {
        Machine::new(
            tape.trim()
                .split(',')
                .map(|x| x.parse::<isize>().unwrap())
                .collect(),
        )
    }

    fn exec_instruction_and_move(&mut self) {
        let opcode = self.tape[self.pos] % 100;
        // Compute modes/parameters. We compute the maximum amount, even if operation takes less,
        // the superfluos ones are just 0 in that case.
        let mode_par1 = ((self.tape[self.pos] / 100) % 10) as usize;
        let mode_par2 = ((self.tape[self.pos] / 1000) % 10) as usize;
        //let mode_par3 = (self.tape[self.pos] / 10000) % 10;
        // +-> Currently not necessary, par3 is always written to.
        let par1 = if self.tape.len() > self.pos + 1 {
            self.tape[self.pos + 1]
        } else {
            0
        };
        let par2 = if self.tape.len() > self.pos + 2 {
            self.tape[self.pos + 2]
        } else {
            0
        };
        let par3 = if self.tape.len() > self.pos + 3 {
            self.tape[self.pos + 3]
        } else {
            0
        };
        match opcode {
            1 => {
                self.tape[par3 as usize] = self.add(par1, mode_par1, par2, mode_par2);
                self.pos += 4;
            }
            2 => {
                self.tape[par3 as usize] = self.mul(par1, mode_par1, par2, mode_par2);
                self.pos += 4;
            }
            3 => match self.input(par1) {
                MachineState::Cont => self.pos += 2,
                MachineState::Waiting => self.state = MachineState::Waiting,
                _ => (),
            },
            4 => {
                self.output(par1, mode_par1);
                self.pos += 2;
            }
            5 => {
                self.jump_if_true(par1, mode_par1, par2, mode_par2);
            }
            6 => {
                self.jump_if_false(par1, mode_par1, par2, mode_par2);
            }
            7 => {
                self.tape[par3 as usize] = if self.lt(par1, mode_par1, par2, mode_par2) {
                    1
                } else {
                    0
                };
                self.pos += 4;
            }
            8 => {
                self.tape[par3 as usize] = if self.eq(par1, mode_par1, par2, mode_par2) {
                    1
                } else {
                    0
                };
                self.pos += 4;
            }
            99 => self.state = MachineState::Halt,
            _ => self.state = MachineState::Error,
        }
    }

    fn jump_if_true(&mut self, par1: isize, par1_mode: usize, par2: isize, par2_mode: usize) {
        let value = if par1_mode == 1 {
            par1
        } else {
            self.tape[par1 as usize]
        };
        if value != 0 {
            self.pos = if par2_mode == 1 {
                par2 as usize
            } else {
                self.tape[par2 as usize] as usize
            };
        } else {
            self.pos += 3;
        }
    }

    fn jump_if_false(&mut self, par1: isize, par1_mode: usize, par2: isize, par2_mode: usize) {
        let value = if par1_mode == 1 {
            par1
        } else {
            self.tape[par1 as usize]
        };
        if value == 0 {
            self.pos = if par2_mode == 1 {
                par2 as usize
            } else {
                self.tape[par2 as usize] as usize
            };
        } else {
            self.pos += 3;
        }
    }

    fn lt(&self, par1: isize, par1_mode: usize, par2: isize, par2_mode: usize) -> bool {
        let x = if par1_mode == 1 {
            par1
        } else {
            self.tape[par1 as usize]
        };
        let y = if par2_mode == 1 {
            par2
        } else {
            self.tape[par2 as usize]
        };
        x < y
    }

    fn eq(&self, par1: isize, par1_mode: usize, par2: isize, par2_mode: usize) -> bool {
        let x = if par1_mode == 1 {
            par1
        } else {
            self.tape[par1 as usize]
        };
        let y = if par2_mode == 1 {
            par2
        } else {
            self.tape[par2 as usize]
        };
        x == y
    }

    fn input(&mut self, par1: isize) -> MachineState {
        match self.inputs.pop() {
            None => MachineState::Waiting,
            Some(i) => {
                self.tape[par1 as usize] = i;
                MachineState::Cont
            }
        }
    }

    fn output(&mut self, par1: isize, par1_mode: usize) {
        let output = if par1_mode == 1 {
            par1
        } else {
            self.tape[par1 as usize]
        };
        self.outputs.push(output);
    }

    fn add(&mut self, par1: isize, par1_mode: usize, par2: isize, par2_mode: usize) -> isize {
        let x = if par1_mode == 1 {
            par1
        } else {
            self.tape[par1 as usize]
        };
        let y = if par2_mode == 1 {
            par2
        } else {
            self.tape[par2 as usize]
        };
        x + y
    }

    fn mul(&mut self, par1: isize, par1_mode: usize, par2: isize, par2_mode: usize) -> isize {
        let x = if par1_mode == 1 {
            par1
        } else {
            self.tape[par1 as usize]
        };
        let y = if par2_mode == 1 {
            par2
        } else {
            self.tape[par2 as usize]
        };
        x * y
    }

    pub fn run(&mut self) -> () {
        loop {
            match self.state {
                MachineState::Cont => self.exec_instruction_and_move(),
                MachineState::Halt => break, // self.result(),
                MachineState::Waiting => break,
                MachineState::Error => panic!("Insert Error Handling here"),
            }
        }
    }

    pub fn cont(&mut self) {
        self.state = MachineState::Cont;
        self.run();
    }

    pub fn set_input(&mut self, inputs: Vec<isize>) {
        self.inputs = inputs;
    }

    pub fn add_input(&mut self, input: isize) {
        self.inputs.push(input);
    }

    pub fn get_output(&self) -> &Vec<isize> {
        &self.outputs
    }

    pub fn has_halted(&self) -> bool {
        self.state == MachineState::Halt
    }

    pub fn is_waiting(&self) -> bool {
        self.state == MachineState::Waiting
    }

    pub fn replace_tape(&mut self, tape: Vec<isize>) {
        self.tape = tape;
        self.pos = 0;
        self.state = MachineState::Cont;
        self.outputs = Vec::new();
    }
}

//fn main() {
//    let tape = include_str!("../input");
//    let tape = parse_tape(tape);
//    let mut computer = Machine::new(tape.clone());
//
//    // Part 1: Exec with 1
//    computer.set_input(vec![1]);
//    computer.run();
//    println!("Output part 1: {:?}", computer.get_output());
//
//    // Part 2: Exec with 5
//    computer.replace_tape(tape.clone());
//    computer.set_input(vec![5]);
//    computer.run();
//    println!("Output part 2: {:?}", computer.get_output());
//}
