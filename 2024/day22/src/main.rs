use std::collections::HashSet;
use std::env;
use std::error::Error;
use std::ffi::OsString;
use std::fs;
use std::time::Instant;

fn get_first_arg() -> Result<OsString, Box<dyn Error>> {
    match env::args_os().nth(1) {
        None => Err(From::from("expected 1 argument, but got none")),
        Some(file_path) => Ok(file_path),
    }
}

fn mix_nb(nb: usize, nb2: usize) -> usize {
    return nb ^ nb2;
}

fn prune_nb(nb: usize) -> usize {
    return nb % 16777216;
}

fn get_next_secret_nb(nb: usize) -> usize {
    let mut secret = nb;
    secret = prune_nb(mix_nb(secret, secret * 64));
    secret = prune_nb(mix_nb(secret, secret / 32));
    secret = prune_nb(mix_nb(secret, secret * 2048));

    secret
}

fn get_secret_nb_sum(secrets: &Vec<usize>, iterations: usize) -> usize {
    let mut sum = 0;
    for s in secrets {
        let mut secret = *s;
        for _ in 0..iterations {
            secret = get_next_secret_nb(secret);
        }
        //println!("{s} -> {secret}");
        sum += secret;
    }

    sum
}

fn get_most_bananas(secrets: &Vec<usize>, iterations: usize) -> usize {
    let mut bananas = 0;
    let mut value_map: Vec<Vec<usize>> = vec![Vec::new(); 19_usize.pow(4)];
    for s in secrets {
        let mut secret = *s;
        let mut trade_values: Vec<usize> = Vec::new();
        let mut idxs = HashSet::new();
        trade_values.push(secret % 10);
        for _ in 0..iterations {
            secret = get_next_secret_nb(secret);
            trade_values.push(secret % 10);
        }
        for i in 4..trade_values.len() {
            let x = ((trade_values[i - 3] as i8 - trade_values[i - 4] as i8) + 9) as usize;
            let y = ((trade_values[i - 2] as i8 - trade_values[i - 3] as i8) + 9) as usize;
            let z = ((trade_values[i - 1] as i8 - trade_values[i - 2] as i8) + 9) as usize;
            let k = ((trade_values[i] as i8 - trade_values[i - 1] as i8) + 9) as usize;

            let index = x * 19_usize.pow(3) + y * 19_usize.pow(2) + z * 19 + k;
            if !idxs.contains(&index) {
                value_map[index].push(trade_values[i]);
                idxs.insert(index);
            }
        }
    }

    value_map.iter().for_each(|x| {
        let b: usize = x.iter().sum();
        if b > bananas {
            bananas = b;
        }
    });

    bananas
}
fn main() -> Result<(), Box<dyn Error>> {
    let file_path = get_first_arg()?;

    let mut vendors: Vec<_> = Vec::new();

    for line in fs::read_to_string(file_path)?.lines() {
        vendors.push(line.parse::<usize>()?);
    }

    let start = Instant::now();
    let secret_sum = get_secret_nb_sum(&vendors, 2000);
    let duration = start.elapsed();
    println!(
        "Part1: {secret_sum} | {}s {}ms {}µs {}ns",
        duration.as_secs(),
        duration.subsec_millis(),
        duration.subsec_micros() % 1000,
        duration.subsec_nanos() % 1000
    );

    let start = Instant::now();
    let bananas = get_most_bananas(&vendors, 2000);
    let duration = start.elapsed();
    println!(
        "Part2: {bananas} | {}s {}ms {}µs {}ns",
        duration.as_secs(),
        duration.subsec_millis(),
        duration.subsec_micros() % 1000,
        duration.subsec_nanos() % 1000
    );

    Ok(())
}
