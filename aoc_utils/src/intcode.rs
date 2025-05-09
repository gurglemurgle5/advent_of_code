#[derive(Clone, Debug)]
pub struct Intcode {
    memory: Vec<i64>,
    state: IntcodeState,
    position: usize,
}

impl Intcode {
    #[must_use]
    pub fn new(memory: Vec<i64>) -> Intcode {
        Intcode {
            memory,
            state: IntcodeState::Running,
            position: 0,
        }
    }

    pub fn step(&mut self) -> IntcodeState {
        if self.state != IntcodeState::Running {
            return self.state;
        }

        let opcode = self.read();

        match opcode {
            1 => {
                // add
                let read_addr_1: usize = self.read().try_into().unwrap();
                let read_addr_2: usize = self.read().try_into().unwrap();
                let write_addr: usize = self.read().try_into().unwrap();
                self.memory[write_addr] = self.memory[read_addr_1] + self.memory[read_addr_2];
            }
            2 => {
                // mul
                let read_addr_1: usize = self.read().try_into().unwrap();
                let read_addr_2: usize = self.read().try_into().unwrap();
                let write_addr: usize = self.read().try_into().unwrap();
                self.memory[write_addr] = self.memory[read_addr_1] * self.memory[read_addr_2];
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

    fn read(&mut self) -> i64 {
        let val = self.memory[self.position];
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
    Running,
    Halted,
}
