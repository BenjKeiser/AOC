use std::env;
use std::error::Error;
use std::ffi::OsString;
use std::fmt;
use std::fs;

#[derive(Debug, Clone, PartialEq)]
enum BlockType {
    EMPTY,
    FILE,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Block {
    t: BlockType,
    idx: u64,
    start: u64,
    length: u32,
}

impl fmt::Display for Block {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "({:?}, {}, {}, {})",
            self.t, self.idx, self.start, self.length
        )
    }
}

fn get_first_arg() -> Result<OsString, Box<dyn Error>> {
    match env::args_os().nth(1) {
        None => Err(From::from("expected 1 argument, but got none")),
        Some(file_path) => Ok(file_path),
    }
}

fn parse_disk_map(dm: &[u32]) -> Vec<Block> {
    let mut pdm: Vec<Block> = Vec::new();
    let mut file_idx: u64 = 0;
    let mut cur_pos: u64 = 0;
    for i in 0..dm.len() {
        let l = dm[i];
        if i % 2 == 0 {
            //even numbers are File Blocks
            let b = Block {t: BlockType::FILE, idx: file_idx, start: cur_pos, length: l};
            pdm.push(b);
            file_idx += 1;
        }
        else {
            //uneven numbers are Empty Blocks
            let b = Block {t: BlockType::EMPTY, idx: 0, start: cur_pos, length: l};
            pdm.push(b);
        }
        cur_pos += l as u64;
    }
    //println!("{:?}", pdm);
    pdm
}

fn compact_fs(pdm: &mut Vec<Block>) {
    let mut f_idx: usize = 0;
    let mut b_idx: usize = pdm.len() - 1;

    while f_idx <= b_idx {
        if pdm[f_idx].t == BlockType::EMPTY && pdm[b_idx].t == BlockType::FILE
        {
            //compact
            pdm[f_idx].t = BlockType::FILE;
            pdm[f_idx].idx = pdm[b_idx].idx;
            let len: i32 = pdm[f_idx].length as i32 - pdm[b_idx].length as i32;
            if len == 0 {
                //FILE Block matches perfectly into EMPTY Block -> we can mark the File block as empty
                pdm[b_idx].t = BlockType::EMPTY;                
            }
            else if len < 0 {
                //FILE Block is larger than EMPTY Block, we Copy what we can from the back and create a new EMPTY block for the copied amount
                pdm.insert(b_idx + 1, Block{t: BlockType::EMPTY, idx: 0, start: pdm[b_idx].start + (pdm[b_idx].length - pdm[f_idx].length) as u64, length: pdm[f_idx].length });

                pdm[b_idx].length -= pdm[f_idx].length;
            }
            else {
                //EMPTY block is larger than FILE Block, we Copy the FILE block and create a new EMPTY block from the rest
                //Note: The new block moves the back index one back
                pdm.insert(f_idx + 1, Block {t: BlockType::EMPTY, idx: 0, start: pdm[f_idx].start + pdm[b_idx].length as u64, length: pdm[f_idx].length - pdm[b_idx].length});
                
                b_idx += 1;
                pdm[b_idx].t = BlockType::EMPTY;
                pdm[f_idx].length = pdm[b_idx].length;
            }
        }
        else {
            while pdm[f_idx].t != BlockType::EMPTY && f_idx < pdm.len() - 1{
                //navigate to empty block
                f_idx += 1;
            }
        
            while pdm[b_idx].t != BlockType::FILE && b_idx > 0 {
                //navigate to file block
                b_idx -= 1;
            }            
        }
    }
}

fn get_checksum(pdm: &[Block]) -> u64 {
    let mut chk_sum = 0;

    for i in 0..pdm.len() {
        if pdm[i].t == BlockType::EMPTY {
            break;
        }
        else {
            for cnt in 0..pdm[i].length {
                //println!("{} * {}", pdm[i].idx, (pdm[i].start + cnt as u64));
                chk_sum += pdm[i].idx * (pdm[i].start + cnt as u64);
            }
        }
    }

    chk_sum
}

fn main() -> Result<(), Box<dyn Error>> {
    let file_path = get_first_arg()?;
    const RADIX: u32 = 10;
    for line in fs::read_to_string(file_path)?.lines() {
        let disk_map: Vec<u32> = line.chars().map(|c| c.to_digit(RADIX).unwrap()).collect();
        //println!("{:?}", disk_map);
        let mut pdm = parse_disk_map(&disk_map);
        //println!("{:?}", pdm);
        compact_fs(&mut pdm);
        //println!("{:?}", pdm);
        let chk_sum = get_checksum(&pdm);
        println!("part1: {chk_sum}");
    }

    Ok(())
}
//1711 for part2
