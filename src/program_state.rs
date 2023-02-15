pub struct ProgramState {
    program: Vec<u16>,
    pc: u32,
}

impl ProgramState {
    pub fn new(program: Vec<u16>) -> Self
    {
        ProgramState {
            program,
            pc: 0,
        }
    }

    pub fn progress_state(&mut self)
    {
        self.pc += 1;
    }

    pub fn get_position(&self) -> u32
    {
        self.pc
    }

    pub fn get_value(&self) -> u16
    {
        self.program[(self.pc as usize)]
    }

    pub fn set_position(&mut self, value: u32)
    {
        self.pc = value - 1;
    }

    pub fn is_empty(&self) -> bool
    {
        if (self.pc as usize) == self.program.len() {
            return true;
        }

        return false;
    }

    pub fn get_next_argument(&mut self) -> u16
    {
        self.progress_state();
        self.get_value()
    }
}
