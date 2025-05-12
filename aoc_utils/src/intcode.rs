#[derive(Clone, Debug)]
pub struct Intcode {
    memory: Vec<i64>,
    state: IntcodeState,
    position: usize,
    io_addr: usize,
}

impl Intcode {
    #[must_use]
    pub fn new(memory: Vec<i64>) -> Intcode {
        Intcode {
            memory,
            state: IntcodeState::Running,
            position: 0,
            io_addr: 0,
        }
    }

    pub fn read_to_vec<T: AsRef<std::path::Path>>(path: T) -> Vec<i64> {
        std::fs::read_to_string(path)
            .unwrap()
            .trim()
            .split(',')
            .map(|num| num.parse().unwrap())
            .collect()
    }

    pub fn step(&mut self) -> IntcodeState {
        if self.state != IntcodeState::Running {
            return self.state;
        }

        let opcode = self.memory[self.position];
        self.position += 1;

        let mut modes = opcode / 100;
        let opcode = opcode % 100;

        match opcode {
            1 => {
                // add
                let read_addr_1: usize = self.read(&mut modes).try_into().unwrap();
                let read_addr_2: usize = self.read(&mut modes).try_into().unwrap();
                let write_addr: usize = self.read(&mut modes).try_into().unwrap();
                self.memory[write_addr] = self.memory[read_addr_1] + self.memory[read_addr_2];
            }
            2 => {
                // mul
                let read_addr_1: usize = self.read(&mut modes).try_into().unwrap();
                let read_addr_2: usize = self.read(&mut modes).try_into().unwrap();
                let write_addr: usize = self.read(&mut modes).try_into().unwrap();
                self.memory[write_addr] = self.memory[read_addr_1] * self.memory[read_addr_2];
            }
            3 => {
                // input
                self.state = IntcodeState::Input;
                self.io_addr = self.read(&mut modes).try_into().unwrap();
            }
            4 => {
                // output
                self.state = IntcodeState::Output;
                self.io_addr = self.read(&mut modes).try_into().unwrap();
            }
            5 => {
                // jump if true
                let condition_addr: usize = self.read(&mut modes).try_into().unwrap();
                let position_addr: usize = self.read(&mut modes).try_into().unwrap();

                if self.memory[condition_addr] != 0 {
                    self.position = self.memory[position_addr].try_into().unwrap();
                }
            }
            6 => {
                // jump if false
                let condition_addr: usize = self.read(&mut modes).try_into().unwrap();
                let position_addr: usize = self.read(&mut modes).try_into().unwrap();

                if self.memory[condition_addr] == 0 {
                    self.position = self.memory[position_addr].try_into().unwrap();
                }
            }
            7 => {
                // less than
                let left_addr: usize = self.read(&mut modes).try_into().unwrap();
                let right_addr: usize = self.read(&mut modes).try_into().unwrap();
                let write_addr: usize = self.read(&mut modes).try_into().unwrap();

                if self.memory[left_addr] < self.memory[right_addr] {
                    self.memory[write_addr] = 1;
                } else {
                    self.memory[write_addr] = 0;
                }
            }
            8 => {
                // equals
                let left_addr: usize = self.read(&mut modes).try_into().unwrap();
                let right_addr: usize = self.read(&mut modes).try_into().unwrap();
                let write_addr: usize = self.read(&mut modes).try_into().unwrap();

                if self.memory[left_addr] == self.memory[right_addr] {
                    self.memory[write_addr] = 1;
                } else {
                    self.memory[write_addr] = 0;
                }
            }
            99 => self.state = IntcodeState::Halted,
            _ => panic!("Unknown opcode: {opcode}"),
        }

        self.state
    }

    pub fn step_until_done(&mut self) -> IntcodeState {
        if self.state != IntcodeState::Running {
            return self.state;
        }

        while self.step() == IntcodeState::Running {}

        self.state
    }

    pub fn input(&mut self, value: i64) {
        assert_eq!(self.state, IntcodeState::Input, "not taking input rn sorry");
        self.memory[self.io_addr] = value;
        self.state = IntcodeState::Running;
    }

    pub fn output(&mut self) -> i64 {
        assert_eq!(self.state, IntcodeState::Output, "no output rn sorry");
        self.state = IntcodeState::Running;
        self.memory[self.io_addr]
    }

    fn read(&mut self, modes: &mut i64) -> i64 {
        let val = match *modes % 10 {
            0 => self.memory[self.position],
            1 => self.position.try_into().unwrap(),
            _ => panic!("unknown mode: {}", *modes % 10),
        };
        *modes /= 10;

        self.position += 1;
        val
    }

    #[must_use]
    pub fn memory(&self) -> &[i64] {
        &self.memory
    }

    #[must_use]
    pub fn state(&self) -> IntcodeState {
        self.state
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntcodeState {
    /// Running normally
    Running,
    /// Program has ended
    Halted,
    /// Program needs input to continue
    Input,
    /// Program has output, must read it
    Output,
}
