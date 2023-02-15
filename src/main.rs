use opcodes::*;
use storage::*;
use program_state::ProgramState;

use std::{process::exit};

mod opcodes;
mod storage;
mod program_state;
mod tests;

fn main() {
    let program_data: Vec<u16> = std::fs::read("target/debug/synacorchallenge.bin")
        .unwrap()
        .chunks_exact(2)
        .into_iter()
        .map(|a| u16::from_le_bytes([a[0], a[1]]))
        .collect();

    let mut state = ProgramState::new(program_data);
    let registers = [Register::new(); 8];
    let mut stack = Stack::new();
    let mut memory = Memory::new();
    
    loop {
        if state.is_empty() {
            break;
        }

        let opcode: u16 = state.get_value();
        match opcode
        {
            0 => halt(),
            1 => {
                let register_number = (state.get_next_argument() - 32768) as usize;
                let mut register = registers[register_number];
                let value = get_value_if_register(registers, state.get_next_argument());
                dbg!("set:", opcode, register_number, value);
                set(&mut register, value);
            },
            2 => {
                let value = get_value_if_register(registers, state.get_next_argument());
                dbg!("push:", value);
                push(&mut stack, value);
            },
            3 => {
                let mut register = registers[(state.get_next_argument() - 32768) as usize];
                dbg!("pop");
                pop(&mut stack, &mut register);
            },
            4 => {
                let mut register = registers[(state.get_next_argument() - 32768) as usize];
                let value = get_value_if_register(registers, state.get_next_argument());
                let value1 = get_value_if_register(registers, state.get_next_argument());
                dbg!("equal:", value, value1);
                equal(&mut register, value, value1);
            }, 
            5 => {
                let mut register = registers[(state.get_next_argument() - 32768) as usize];
                let value = get_value_if_register(registers, state.get_next_argument());
                let value1 = get_value_if_register(registers, state.get_next_argument());
                dbg!("greater_than:", value, value1);
                greater_than(&mut register, value, value1);
            },
            6 => {
                let value = get_value_if_register(registers, state.get_next_argument());
                dbg!("jump:", value);
                jump(&mut state, value);
            },
            7 => {
                let value = get_value_if_register(registers, state.get_next_argument());
                let dest = state.get_next_argument();
                dbg!("jump_if_true:", value, dest);
                jump_if_true(&mut state, value, dest);
            },
            8 => {
                let value = get_value_if_register(registers, state.get_next_argument());
                let dest = get_value_if_register(registers, state.get_next_argument());
                dbg!("jump_if_false:", value, dest);
                jump_if_false(&mut state, value, dest);
            },
            9 => {
                let mut register = registers[(state.get_next_argument() - 32768) as usize];
                let value = get_value_if_register(registers, state.get_next_argument());
                let value1 = get_value_if_register(registers, state.get_next_argument());
                dbg!("add:", value, value1);
                add(&mut register, value, value1);
            },
            10 => {
                let mut register = registers[(state.get_next_argument() - 32768) as usize];
                let value = get_value_if_register(registers, state.get_next_argument());
                let value1 = get_value_if_register(registers, state.get_next_argument());
                dbg!("multiply:", value, value1);
                multiply(&mut register, value, value1);
            },
            11 => {
                let mut register = registers[(state.get_next_argument() - 32768) as usize];
                let value = get_value_if_register(registers, state.get_next_argument());
                let value1 = get_value_if_register(registers, state.get_next_argument());
                dbg!("save_mod:", value, value1);
                save_mod(&mut register, value, value1)
            },
            12 => {
                let mut register = registers[(state.get_next_argument() - 32768) as usize];
                let value = get_value_if_register(registers, state.get_next_argument());
                let value1 = get_value_if_register(registers, state.get_next_argument());
                dbg!("save_and:", value, value1);
                save_and(&mut register, value, value1)
            },
            13 => {
                let mut register = registers[(state.get_next_argument() - 32768) as usize];
                let value = get_value_if_register(registers, state.get_next_argument());
                let value1 = get_value_if_register(registers, state.get_next_argument());
                dbg!("save_or:", value, value1);
                save_or(&mut register, value, value1)
            },
            14 => {
                let mut register = registers[(state.get_next_argument() - 32768) as usize];
                let value = get_value_if_register(registers, state.get_next_argument());
                dbg!("save_not:", value);
                save_not(&mut register, value)
            },
            15 => {
                let mut register = registers[(state.get_next_argument() - 32768) as usize];
                let address = get_value_if_register(registers, state.get_next_argument());
                dbg!("read_mem:", address);
                read_mem(&mut memory, &mut register, address);
            },
            16 => {
                let address = get_value_if_register(registers, state.get_next_argument());
                let register = registers[(state.get_next_argument() - 32768) as usize];
                dbg!("write_mem:", address, register.get());
                write_mem(&mut memory, address, register.get())
            },
            17 => {
                let value = get_value_if_register(registers, state.get_next_argument());
                dbg!("call:", value);
                call(&mut stack, (state.get_position() + 1) as u16);
                jump(&mut state, value);
            },
            18 => {
                let return_address = ret(&mut stack);
                dbg!("ret:", return_address);
                state.set_position(return_address as u32);
            },
            19 => {
                let character = state.get_next_argument();
                dbg!("char_out", (character as u8) as char);
                char_out(character as u8)
            },
            20 => {
                char_in();
            }
            21 => noop(), // noop do nothing

            code => {
                println!("Unknown opcode: {code}");
                exit(1);
            },
        }

        state.progress_state();
    }
}

fn get_value_if_register(registers: [Register; 8], value: u16) -> u16
{
    // If is a register
    if value > 32767 {
        let register = registers[(value - 32768) as usize];
        dbg!("Reading register for value", (value - 32768) as usize);
        return register.get();
    }

    value
}
