// Import the standard library's I/O module so we can read from stdin.
use std::{
    env,
    error::Error,
    ffi::OsString,
    process,
};
use csv::ReaderBuilder;

fn get_first_arg() -> Result<OsString, Box<dyn Error>> {
    match env::args_os().nth(1) {
        None => Err(From::from("expected 1 argument, but got none")),
        Some(file_path) => Ok(file_path),
    }
}

// 0: undef; -1: decreasing; 1: increasing
fn get_dir(v1: i32, v2: i32) -> i32 {
    let mut dir = 0;
    if v2 < v1
    {
        dir = -1;
    }
    else if v2 > v1
    {
        dir = 1;
    }
    dir
}

fn check_val(v1: i32, v2: i32) -> i32 {
    let mut ret = 1;
    let diff = (v2 - v1).abs();
    
    if (diff < 1) || (diff > 3)
    {
        ret = 0;
    }
    if v2 > v1
    {
        ret = 0;              
    }

    ret
}

fn analyze_record(record: &Vec<i32>, mut cnt: i32) -> i32 {
    let mut ret = 1;
    let mut pos = 0;
    
    let dir = get_dir(record[0], record[1]);
    if dir == 0
    {
        ret = 0;
    }

    for v in 1..record.len()
    {
        let val = record[v];
        let prev_val = record[v-1];
        
        if dir > 0
        {
            ret = check_val(val, prev_val);
        }
        else if dir < 0 
        {
            ret = check_val(prev_val, val);
        }

        if ret == 0
        {
            pos = v;
            break;
        }
    }

    if (ret == 0) && (cnt < 1)
    {
        //println!("{:?}", record);
        cnt += 1;
        let mut r = record.to_vec();
        r.remove(pos);
        ret = analyze_record(&r, cnt);

        if pos >= 1 && ret == 0
        {
            r = record.to_vec();
            r.remove(pos-1);
            ret = analyze_record(&r, cnt);
        }

        if pos == 2 && ret == 0
        {
            //special case if initial direction issue
            r = record.to_vec();
            r.remove(pos-2);
            ret = analyze_record(&r, cnt);
        }
    }

    ret
}

fn parse() -> Result<(), Box<dyn Error>> {
    let mut sum1 = 0;
    let mut sum2 = 0;
    let mut rec: Vec<i32> = Vec::new();

    let file_path = get_first_arg()?;

    let mut rdr = ReaderBuilder::new()
        .has_headers(false)
        .delimiter(b' ')
        .flexible(true)
        .from_path(file_path)?;

    // Loop over each record.
    for result in rdr.records() 
    {
        let record = result?;
        for v in record.iter()
        {
            rec.push(v.parse::<i32>().unwrap())
        }
        sum1 += analyze_record(&rec, 1);
        sum2 += analyze_record(&rec, 0);
        rec.clear();
    }
    println!("p1:{:?}", sum1);
    println!("p2:{:?}", sum2);
    Ok(())
}

fn main() {
    if let Err(err) = parse() {
        println!("{}", err);
        process::exit(1);
    }
}