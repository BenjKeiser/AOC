// Import the standard library's I/O module so we can read from stdin.
use std::{
    env,
    error::Error,
    ffi::OsString,
    process,
};
use csv::ReaderBuilder;

type Record = (i32, i32);

fn get_first_arg() -> Result<OsString, Box<dyn Error>> {
    match env::args_os().nth(1) {
        None => Err(From::from("expected 1 argument, but got none")),
        Some(file_path) => Ok(file_path),
    }
}

fn handle_lists_p1(v1: &mut Vec<i32>, v2: &mut Vec<i32>) {
    v1.sort();
    v2.sort();
    let mut sum = 0;
    for i in 0..v1.len() {
        sum += (v1[i] - v2[i]).abs();
    }
    println!("{:?}", sum);
}

fn handle_lists_p2(v1: &mut Vec<i32>, v2: &mut Vec<i32>) {
    v1.sort();
    v2.sort();
    let mut sum = 0;
    for i in 0..v1.len() {
        sum += v1[i] * (v2.iter().filter(|&n| *n == v1[i]).count() as i32);
    }
    println!("{:?}", sum);
}

fn parse() -> Result<(), Box<dyn Error>> {
    let mut list1: Vec<i32> = Vec::new();
    let mut list2: Vec<i32> = Vec::new();
    let file_path = get_first_arg()?;
    let mut rdr = ReaderBuilder::new()
        .has_headers(false)
        .delimiter(b' ')
        .from_path(file_path)?;
    // Loop over each record.
    for result in rdr.deserialize() {
        let record: Record = result?;
        list1.push(record.0);
        list2.push(record.1);
        println!("{:?}, {:?}", record.0, record.1);
    }
    handle_lists_p1(&mut list1,&mut list2);
    handle_lists_p2(&mut list1,&mut list2);
    Ok(())
}

fn main() {
    if let Err(err) = parse() {
        println!("{}", err);
        process::exit(1);
    }
}