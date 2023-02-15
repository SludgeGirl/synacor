use crate::{storage::{Register, Stack, Memory}, program_state::ProgramState};
use std::{process::exit};

pub fn halt()
{
    println!("HALTING");
    exit(0);
}

pub fn set(register: &mut Register, value: u16)
{
    register.set(value);
}

pub fn push(stack: &mut Stack, value: u16)
{
    stack.push(value);
}

pub fn pop(stack: &mut Stack, register: &mut Register)
{
    register.set(stack.pop().unwrap());
}

pub fn equal(register: &mut Register, value: u16, value1: u16)
{
    if value == value1 {
        register.set(1);
    } else {
        register.set(0);
    }
}

pub fn greater_than(register: &mut Register, value: u16, value1: u16)
{
    if value > value1 {
        register.set(1);
    } else {
        register.set(0);
    }
}

pub fn jump(pc: &mut ProgramState, value: u16)
{
    pc.set_position(value as u32);
}

// JT
pub fn jump_if_true(pc: &mut ProgramState, value: u16, dest: u16)
{
    if value != 0 {
        pc.set_position(dest as u32);
    }
}

pub fn jump_if_false(pc: &mut ProgramState, value: u16, dest: u16)
{
    if value == 0 {
        pc.set_position(dest as u32);
    }
}

pub fn add(register: &mut Register, value: u16, value1: u16)
{
    let mut sum = value + value1;
    if sum > 32768 {
        sum = sum - 32768;
    }
    register.set(sum);
}

pub fn multiply(register: &mut Register, value: u16, value1: u16)
{
    let mut total = value * value1;
    if total > 32768 {
        total = total - 32768;
    }
    register.set(total);
}

pub fn save_mod(register: &mut Register, value: u16, value1: u16)
{
    register.set(value % value1);
}

pub fn save_and(register: &mut Register, value: u16, value1: u16)
{
    register.set(value & value1);
}

pub fn save_or(register: &mut Register, value: u16, value1: u16)
{
    register.set(value | value1);
}

pub fn save_not(register: &mut Register, value: u16)
{
    register.set(!value);
}

pub fn read_mem(memory: &Memory, register: &mut Register, address: u16)
{
    register.set(memory.read(address));
}

pub fn write_mem(memory: &mut Memory, address: u16, value: u16)
{
    memory.write(address, value);
}

pub fn call(stack: &mut Stack, address: u16)
{
    stack.push(address);
}

pub fn ret(stack: &mut Stack) -> u16
{
    let address = stack.pop().unwrap();
    return address;
}

pub fn char_out(character: u8)
{
    print!("{}", character as char);
}

pub fn char_in()
{
    println!("Panicking, reached char_in");
    exit(1);
}

pub fn noop()
{} // Do nothing
