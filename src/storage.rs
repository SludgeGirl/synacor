#[derive(Clone, Copy)]
pub struct Register
{
    value: u16,
}

impl Register
{
    pub fn set(&mut self, value: u16)
    {
        self.value = value;
        dbg!("Set register value:", self.value);
    }

    pub fn get(&self) -> u16
    {
        dbg!("Get register value:", self.value);
        return self.value;
    }

    pub fn new() -> Register
    {
        Register {
            value: 0
        }
    }
}

pub struct Stack
{
    stack: Vec<u16>,
}

impl Stack
{
    pub fn push(&mut self, value: u16)
    {
        self.stack.push(value);
    }

    pub fn pop(&mut self) -> Option<u16>
    {
        return self.stack.pop();
    }

    pub fn new() -> Stack
    {
        Stack { stack: Vec::new() }
    }
}

pub struct Memory
{
    memory: Vec<u16>,
}

impl Memory 
{
    pub fn write(&mut self, address: u16, value: u16)
    {
        self.memory[(address / 16) as usize] = value;
    }

    pub fn read(&self, address: u16) -> u16
    {
        return self.memory[(address / 16) as usize];
    }

    pub fn new() -> Self
    {
        Memory {
            memory: Vec::new(),
        }
    }
}
