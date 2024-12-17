use std::convert::TryFrom;
use std::env;
use std::error::Error;
use std::ffi::OsString;
use std::fmt;
use std::fs;
use std::time::Instant;
use std::u64::MAX;

// OpCode reads operand after it then instruction pointer is increased by 2
#[derive(Debug, Clone, PartialEq)]
#[repr(u8)]

enum OpCode {
    ADV = 0, // Reg A = ( Reg A / 2 pow(combo operand)) -> 1 << combo
    BXL = 1, // Reg B = ( Reg B ^ (literal operand)) ^ <- Bitwise XOR
    BST = 2, // Reg B = (combo operand) % 8
    JNZ = 3, // if Reg A == 0 { do nothing } else { jump literal operand } <- do not increase instruction pointer
    BXC = 4, // Reg B = ( Reg B ^ Reg C) <- Reads operand but ignores it
    OUT = 5, // outputs combo operand % 8 -> outputs comma separated
    BDV = 6, // Reg B = ( Reg A / 2 sqr(combo operand))
    CDV = 7, // Reg C = ( Reg A / 2 sqr(combo operand))
}

// Literal Operand = actual value
// Combo Operand:
// - Combo operands 0 through 3 represent literal values 0 through 3.
// - Combo operand 4 represents the value of register A.
// - Combo operand 5 represents the value of register B.
// - Combo operand 6 represents the value of register C.
// - Combo operand 7 is reserved and will not appear in valid programs.
impl TryFrom<u8> for OpCode {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x00 => Ok(OpCode::ADV),
            0x01 => Ok(OpCode::BXL),
            0x02 => Ok(OpCode::BST),
            0x03 => Ok(OpCode::JNZ),
            0x04 => Ok(OpCode::BXC),
            0x05 => Ok(OpCode::OUT),
            0x06 => Ok(OpCode::BDV),
            0x07 => Ok(OpCode::CDV),
            _ => Err("Invalid OpCode"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Computer {
    a: u64,
    b: u64,
    c: u64,
    prog: Vec<u8>,
}

impl fmt::Display for Computer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "RegA: {}", self.a)?;
        writeln!(f, "RegB: {}", self.b)?;
        writeln!(f, "RegC: {}", self.c)?;
        writeln!(f, "Prog: {:?}", self.prog)?;
        Ok(())
    }
}

impl Computer {
    fn get_combo_op(self: &Self, idx: usize) -> u64 {
        match self.prog[idx] {
            4 => return self.a,
            5 => return self.b,
            6 => return self.c,
            7 => return MAX,
            _ => return self.prog[idx] as u64,
        }
    }

    fn get_copy(self: &Self) -> u64 {
        let mut out: Vec<u8>;
        let mut idx: u64;
        let mut reg_a = 8_u64.pow(self.prog.len() as u32 - 1);  // digits of the solution increase with a factor of 8
        loop {
            let mut comp = self.clone();
            comp.a = reg_a;

            out = comp.run_prog();
            if out.len() != self.prog.len(){
                println!("This is not expected! {reg_a}");
                println!("out: {:?}", out);
                println!("Prog: {:?}", self.prog);
                break;
            }
            else {
                if out == self.prog {
                    break;
                }
                idx = 0;
                for (i, el) in out.iter().enumerate() {
                    if *el != self.prog[i] {
                        idx += 8_u64.pow(i as u32) / 8;
                    }
                }
            }
            reg_a += idx.max(1);
        }
        reg_a
    }

    fn run_prog(self: &mut Self) -> Vec<u8> {
        let mut out: Vec<u8> = Vec::new();
        let mut instr_ptr: usize = 0;
        let mut jump: bool = false;

        while instr_ptr < self.prog.len() {
            match OpCode::try_from(self.prog[instr_ptr]).unwrap() {
                OpCode::ADV => {
                    let op = self.get_combo_op(instr_ptr + 1);
                    self.a = self.a / (1 << op); // 2 to the power of op
                }
                OpCode::BXL => {
                    self.b = self.b ^ self.prog[instr_ptr + 1] as u64;
                }
                OpCode::BST => {
                    self.b = self.get_combo_op(instr_ptr + 1) % 8;
                }
                OpCode::JNZ => {
                    if self.a != 0 {
                        instr_ptr = self.prog[instr_ptr + 1] as usize;
                        jump = true;
                    }
                }
                OpCode::BXC => {
                    self.b = self.b ^ self.c;
                }
                OpCode::OUT => {
                    let op = self.get_combo_op(instr_ptr + 1);
                    out.push((op % 8) as u8);
                }
                OpCode::BDV => {
                    let op = self.get_combo_op(instr_ptr + 1);
                    self.b = self.a / (1 << op); // 2 to the power of op
                }
                OpCode::CDV => {
                    let op = self.get_combo_op(instr_ptr + 1);
                    self.c = self.a / (1 << op); // 2 to the power of op
                }
            }
            if !jump {
                instr_ptr += 2;
            }
            jump = false;
        }

        out
    }
}

fn get_first_arg() -> Result<OsString, Box<dyn Error>> {
    match env::args_os().nth(1) {
        None => Err(From::from("expected 1 argument, but got none")),
        Some(file_path) => Ok(file_path),
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let file_path = get_first_arg()?;
    let mut comp: Computer = Computer {
        a: 0,
        b: 0,
        c: 0,
        prog: Vec::new(),
    };
    for line in fs::read_to_string(file_path)?.lines() {
        let str: Vec<&str> = line.splitn(2, ':').collect();
        match str[0] {
            "Register A" => {
                comp.a = str[1].trim().parse::<u64>()?;
            }
            "Register B" => {
                comp.b = str[1].trim().parse::<u64>()?;
            }
            "Register C" => {
                comp.c = str[1].trim().parse::<u64>()?;
            }
            "Program" => {
                for c in str[1].trim().split(',') {
                    comp.prog.push(c.parse::<u8>()?);
                }
            }
            _ => {}
        }
    }
    println!("{comp}");

    let comp2 = comp.clone();

    let start = Instant::now();
    let out = comp.run_prog();
    let duration = start.elapsed();
    let mut out_str = String::new();
    for o in &out {
        out_str += &o.to_string();
        out_str += ",";
    }
    out_str.pop(); // remove the last comma
    println!("Part1: {out_str} | {}s {}ms {}µs {}ns", duration.as_secs(), duration.subsec_millis(), duration.subsec_micros() % 1000, duration.subsec_nanos() % 1000);

    let start = Instant::now();
    let ra = comp2.get_copy();
    let duration = start.elapsed();
    println!("Part2: {ra} | {}s {}ms {}µs {}ns", duration.as_secs(), duration.subsec_millis(), duration.subsec_micros() % 1000, duration.subsec_nanos() % 1000);

    Ok(())
}
