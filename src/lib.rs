mod brainfuck;

use brainfuck::{BrainfuckContext, Instruction};
use std::error::Error;
use std::fs;

pub fn run(program_path: &str) -> Result<(), Box<dyn Error>> {
    let program_string = fs::read_to_string(program_path)?;
    let program_parsed = parse(&program_string)?;

    let mut bf_context = BrainfuckContext::new();
    bf_context.execute(program_parsed.as_slice())?;

    Ok(())
}

fn parse(program: &str) -> Result<Vec<Instruction>, Box<dyn Error>> {
    let chars: Vec<char> = program.chars().collect();
    let mut result = Vec::new();

    let mut index = 0;
    while index < chars.len() {
        match chars[index] {
            '+' => {
                let last_index = result.len().wrapping_sub(1);
                if let Some(ins) = result.get_mut(last_index) {
                    if let Instruction::IncreaseCell(n) = ins {
                        *ins = Instruction::IncreaseCell(*n + 1);
                    } else {
                        result.push(Instruction::IncreaseCell(1));
                    }
                } else {
                    result.push(Instruction::IncreaseCell(1));
                }
            },
            '-' => {
                let last_index = result.len().wrapping_sub(1);
                if let Some(ins) = result.get_mut(last_index) {
                    if let Instruction::DecreaseCell(n) = ins {
                        *ins = Instruction::DecreaseCell(*n + 1);
                    } else {
                        result.push(Instruction::DecreaseCell(1));
                    }
                } else {
                    result.push(Instruction::DecreaseCell(1));
                }
            },
            '>' => {
                let last_index = result.len().wrapping_sub(1);
                if let Some(ins) = result.get_mut(last_index) {
                    if let Instruction::NextCell(n) = ins {
                        *ins = Instruction::NextCell(*n + 1);
                    } else {
                        result.push(Instruction::NextCell(1));
                    }
                } else {
                    result.push(Instruction::NextCell(1));
                }
            },
            '<' => {
                let last_index = result.len().wrapping_sub(1);
                if let Some(ins) = result.get_mut(last_index) {
                    if let Instruction::PreviousCell(n) = ins {
                        *ins = Instruction::PreviousCell(*n + 1);
                    } else {
                        result.push(Instruction::PreviousCell(1));
                    }
                } else {
                    result.push(Instruction::PreviousCell(1));
                }
            },
            '.' => result.push(Instruction::PrintCharacter),
            ',' => result.push(Instruction::ReadCharacter),
            '[' => {
                index += 1;
                let loop_start = index;
                let mut false_positives = 0;
                while chars[index] != ']' || false_positives != 0 {
                    match chars[index] {
                        '[' => false_positives += 1,
                        ']' => false_positives -= 1,
                        _ => (),
                    }
                    index += 1;
                }

                let instructions = parse(&program[loop_start..index])?;
                let loop_instruction = Instruction::Loop(instructions);
                result.push(loop_instruction);
            },
            ']' => Err("End of loop without beginning.")?,
            _ => (),
        }

        index += 1;
    }

    Ok(result)
}
