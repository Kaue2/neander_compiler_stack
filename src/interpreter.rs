use clap::Parser;
use std::error::Error;
use std::fmt::{self, write};
use std::fs::{self};
use std::{env, io};

type InstructionFn = fn(&mut Interpreter, addr: usize);
struct ProgramCounter(u16);

impl ProgramCounter {
    fn usize(&self) -> usize {
        self.0 as usize
    }

    fn increment(&mut self) {
        self.0 = self.0.saturating_add(2);
    }
}

pub struct Interpreter {
    pub acc: u8,
    pub zero_f: bool,
    pub negative_f: bool,
    pub mem: Vec<u8>,
    pc: ProgramCounter,
    should_stop: bool,
}

#[derive(Parser)]
struct Cli {
    #[arg(long, short)]
    path: Option<String>,

    #[arg(long, short)]
    pc: Option<u16>,
}

fn nop(_i: &mut Interpreter, _addr: usize) {
    // println!("função nop chamada");
    return;
}

fn sta(i: &mut Interpreter, addr: usize) {
    println!("função sta chamada");
    i.mem[addr] = i.acc;
    return;
}

fn lda(i: &mut Interpreter, addr: usize) {
    println!("função lda chamada para o endereço: {}", addr);
    i.acc = i.mem[addr];
    return;
}

fn add(i: &mut Interpreter, addr: usize) {
    println!("Função soma chamada para o endereço: {}", addr);
    i.acc += i.mem[addr];
    return;
}

fn or(i: &mut Interpreter, addr: usize) {
    i.acc = i.mem[addr] | i.acc;
    return;
}

fn and(i: &mut Interpreter, addr: usize) {
    i.acc = i.mem[addr] & i.acc;
    return;
}

fn not(i: &mut Interpreter, _addr: usize) {
    i.acc = !i.acc;
    return;
}

fn jmp(i: &mut Interpreter, addr: usize) {
    println!("função jmp chamada para o endereço: {}", addr);
    println!("mem: {}", i.mem[addr]);
    i.pc.0 = addr as u16;
    return;
}

fn jn(i: &mut Interpreter, addr: usize) {
    if i.negative_f {
        jmp(i, addr);
    }
    return;
}

fn jz(i: &mut Interpreter, addr: usize) {
    if i.zero_f {
        jmp(i, addr);
    }
    return;
}

fn hlt(i: &mut Interpreter, _addr: usize) {
    i.should_stop = true;
    return;
}

impl Interpreter {
    fn new() -> Self {
        Interpreter {
            acc: 0,
            pc: ProgramCounter(4),
            zero_f: true,
            negative_f: false,
            mem: vec![0],
            should_stop: false,
        }
    }

    fn set_mem(mut self, mem: Vec<u8>) -> Self {
        self.mem = mem;
        return self;
    }

    fn set_pc(mut self, pc: u16) -> Self {
        self.pc.0 = pc;
        return self;
    }

    fn get_rules(opcode: u8) -> Option<InstructionFn> {
        match opcode {
            0 => Some(nop),
            16 => Some(sta),
            32 => Some(lda),
            48 => Some(add),
            64 => Some(or),
            80 => Some(and),
            96 => Some(not),
            128 => Some(jmp),
            144 => Some(jn),
            160 => Some(jz),
            240 => Some(hlt),
            _ => None,
        }
    }

    fn fetch(&mut self) -> u8 {
        let code = self.mem[self.pc.usize()];
        self.pc.increment();
        code
    }

    fn run(&mut self) {
        while self.pc.usize() < self.mem.len() && self.should_stop != true {
            // println!("pc: {}", self.pc.usize() * 2);
            let opcode = self.fetch();

            println!("opcode: {}", opcode);

            if let Some(function) = Interpreter::get_rules(opcode) {
                if opcode == 0 || opcode == 240 {
                    function(self, 0);
                    continue;
                }

                let addr = (self.fetch() as usize) * 2 + 4;
                function(self, addr);
            } else {
                continue;
            }
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
    let cli = Cli::parse();
    let data: Vec<u8>;
    let mut buff = String::new();

    if let Some(p) = cli.path {
        println!("{}", p);
        data = fs::read(p)?;
    } else {
        io::stdin().read_line(&mut buff)?;
        let file_name = buff.trim().replace("\"", "");
        data = fs::read(file_name)?;
    }

    let mut interpreter = Interpreter::new().set_mem(data);

    if let Some(program_counter) = cli.pc {
        interpreter = Interpreter::set_pc(interpreter, program_counter);
    }
    interpreter.run();

    Ok(())
}
