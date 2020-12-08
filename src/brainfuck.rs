use std::collections::VecDeque;
use std::error::Error;
use std::io;
use std::io::Write;

const CELL_AMOUNT: usize = 300000;

#[derive(Debug)]
pub enum Instruction {
    IncreaseCell(usize),
    DecreaseCell(usize),
    NextCell(usize),
    PreviousCell(usize),
    PrintCharacter,
    ReadCharacter,
    Loop(Vec<Instruction>),
}

pub struct BrainfuckContext {
    cells: [u8; CELL_AMOUNT],
    cell_index: usize,
    input_queue: VecDeque<char>,
}

impl BrainfuckContext {
    pub fn new() -> BrainfuckContext {
        BrainfuckContext {
            cells: [0; CELL_AMOUNT],
            cell_index: CELL_AMOUNT / 2,
            input_queue: VecDeque::new(),
        }
    }

    pub fn execute(&mut self, program: &[Instruction]) -> Result<(), Box<dyn Error>> {
        for instruction in program {
            match instruction {
                Instruction::IncreaseCell(n) => {
                    self.cells[self.cell_index] = self.cells[self.cell_index].wrapping_add(*n as u8)
                },
                Instruction::DecreaseCell(n) => {
                    self.cells[self.cell_index] = self.cells[self.cell_index].wrapping_sub(*n as u8)
                },
                Instruction::NextCell(n) => self.cell_index = self.cell_index.wrapping_add(*n),
                Instruction::PreviousCell(n) => self.cell_index = self.cell_index.wrapping_sub(*n),
                Instruction::PrintCharacter => {
                    print!("{}", self.cells[self.cell_index] as char);
                    io::stdout().flush()?;
                },
                Instruction::ReadCharacter => {
                    if self.input_queue.len() == 0 {
                        let stdin = io::stdin();
                        let mut read = String::new();

                        stdin.read_line(&mut read)?;

                        for c in read[..read.len() - 1].chars() {
                            self.input_queue.push_front(c);
                        }
                    }

                    let inp = self.input_queue.pop_back();
                    if let Some(c) = inp {
                        self.cells[self.cell_index] = c as u8;
                    }
                },
                Instruction::Loop(p) => {
                    while self.cells[self.cell_index] != 0 {
                        self.execute(p)?;
                    }
                },
            }
        }

        Ok(())
    }
}
