use std::error::Error;
use std::fmt::{self, write};
use std::fs::{self};

type InstructionFn = fn(&mut Interpreter);
struct ProgramCounter(u8);

impl ProgramCounter {
    fn usize(&self) -> usize {
        self.0 as usize
    }

    fn increment(&mut self) {
        self.0.saturating_add(2);
    }
}

pub struct Interpreter {
    pub acc: u8,
    pub pc: ProgramCounter,
    pub zero_f: bool,
    pub negative_f: bool,
    pub mem: Vec<u8>,
    should_stop: bool,
}

fn nop(_i: &mut Interpreter) {}

fn sta(_i: &mut Interpreter) {}

fn lda(_i: &mut Interpreter) {}

fn add(_i: &mut Interpreter) {}

fn or(_i: &mut Interpreter) {}

fn and(_i: &mut Interpreter) {}

fn not(_i: &mut Interpreter) {}

fn jmp(_i: &mut Interpreter) {}

fn jn(_i: &mut Interpreter) {}

fn jz(_i: &mut Interpreter) {}

fn hlt(_i: &mut Interpreter) {}

impl Interpreter {
    fn new(mem: Vec<u8>) -> Self {
        Interpreter {
            acc: 0,
            pc: ProgramCounter(4),
            zero_f: true,
            negative_f: false,
            mem,
            should_stop: false,
        }
    }

    fn get_rules(opcode: u8) -> InstructionFn {
        match opcode {
            0 => nop,
            16 => sta,
            32 => lda,
            48 => add,
            64 => or,
            80 => and,
            96 => not,
            128 => jmp,
            144 => jn,
            160 => jz,
            240 => hlt,
            _ => panic!("Instrução não conhecida."),
        }
    }

    fn fetch(&mut self) -> u8 {
        let opcode = self.mem[self.pc.usize()];
        self.pc.increment();
        opcode
    }

    fn run(&mut self) {
        while (self.pc.usize() * 2) < self.mem.len() && self.should_stop != true {
            let opcode = self.fetch();

            if let Some(function) = Interpreter::get_rules(opcode) {}
        }
    }
}

impl fmt::Display for Interpreter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Interpreter: \n ACC: {}\n PC: {}\n", self.acc, self.pc.0)?;

        for chunk in self.mem.chunks(16) {
            for byte in chunk {
                write!(f, " {:02}", byte)?;
            }
            writeln!(f)?;
        }

        write!(f, "\n")
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let data = fs::read("file.bin")?;
    let interpreter = Interpreter::new(data);
    println!("{}", interpreter);

    Ok(())
}
