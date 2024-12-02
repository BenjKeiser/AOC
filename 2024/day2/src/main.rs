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

fn analyze_record(record: &csv::StringRecord) -> i32 {
    let mut ret = 1;
    let mut dir = 0; // 0: undef; -1: decreasing; 1: increasing
    let mut old_val = 0;
    let mut cnt = 0;
    for v in record.iter()
    {
        let val = v.parse::<i32>().unwrap();

        if dir == 0
        {
            if cnt != 0
            {
                let diff = (val - old_val).abs();
                if diff < 1
                {
                    ret = 0;
                    break;   
                }

                if diff > 3
                {
                    ret = 0;
                    break;   
                }
                
                if val < old_val
                {
                    dir = -1;
                }
                else if val > old_val
                {
                    dir = 1;
                }
                else
                {
                    ret = 0;
                    break;
                }
            }
        }
        else
        {
            let diff = (val - old_val).abs();
            if diff < 1
            {
                ret = 0;
                break;   
            }

            if diff > 3
            {
                ret = 0;
                break;   
            }

            if dir < 0
            {
                if val > old_val
                {
                    ret = 0;
                    break;                    
                }
            }
            if dir > 0 
            {
                if val < old_val
                {
                    ret = 0;
                    break;                    
                }
            }
        }
        old_val = val;
        cnt += 1;
    }
    ret
}

fn parse() -> Result<(), Box<dyn Error>> {
    let mut sum = 0;
    let file_path = get_first_arg()?;
    let mut rdr = ReaderBuilder::new()
        .has_headers(false)
        .delimiter(b' ')
        .flexible(true)
        .from_path(file_path)?;

    // Loop over each record.
    for result in rdr.records() {
        let record = result?;
        sum += analyze_record(&record);
    }
    println!("{:?}", sum);
    Ok(())
}

fn main() {
    if let Err(err) = parse() {
        println!("{}", err);
        process::exit(1);
    }
}