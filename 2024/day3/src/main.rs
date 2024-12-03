use std::fs;
use std::error::Error;
use std::ffi::OsString;
use std::env;

fn get_first_arg() -> Result<OsString, Box<dyn Error>> {
    match env::args_os().nth(1) {
        None => Err(From::from("expected 1 argument, but got none")),
        Some(file_path) => Ok(file_path),
    }
}

fn parse_string(msg: &String) {
    let message = msg.split("mul(");
    let mut sum = 0;
    for m in message.skip(1) {
        match m.split_once(")") {
            Some((nbs, _rest)) => {
                match nbs.split_once(",") {
                    Some((v1, v2)) => {
                        if v1.chars().all(|x| x.is_ascii_digit()) && v2.chars().all(|x| x.is_ascii_digit())
                        {
                            let nb1 = v1.parse::<i32>().unwrap();
                            let nb2 = v2.parse::<i32>().unwrap();
                            if nb1 <= 999 && nb2 <= 999
                            {
                                sum += nb1 * nb2;
                            }
                        }
                    }
                    None => {

                    }
                }
            }
            None => {

            }
        }
    }
    println!("{}", sum);
}

fn main() -> Result<(), Box<dyn Error>> {
    let file_path = get_first_arg()?;
    let message: String = fs::read_to_string(file_path)?;
    parse_string(&message);
    Ok(())
}