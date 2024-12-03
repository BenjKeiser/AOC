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

fn check_enable(msg: &str, mut en: bool) -> bool {
    let en_str = msg.split("do");
    for e in en_str {
        //check if do()
        match e.find("()") {
            Some(pos) => {
                if pos == 0 {
                    //println!("Enable -> {}", e);
                    en = true;
                }
            }
            None => {}
        }

        //check if don't()
        match e.find("n't()") {
            Some(pos) => {
                if pos == 0 {
                    //println!("Disable -> {}", e);
                    en = false;
                }
            }
            None => {}
        }

    }
    en
}

fn parse_string(msg: &String, check: bool) {
    let message = msg.split("mul(");
    let mut sum = 0;
    let mut enable: bool = true;
    for m in message {
        //println!("{} -> {}", m, enable);
        if enable
        {
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
        if check {
            enable = check_enable(m, enable);
        }
    }
    println!("{}", sum);
}

fn main() -> Result<(), Box<dyn Error>> {
    let file_path = get_first_arg()?;
    let message: String = fs::read_to_string(file_path)?;
    parse_string(&message, false);
    parse_string(&message, true);
    Ok(())
}