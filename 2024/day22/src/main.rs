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
    return nb ^ nb2
}

fn prune_nb(nb: usize) -> usize {
    return nb % 16777216
}

fn get_next_secret_nb(nb: usize) -> usize {
    let mut secret = nb;
    secret = prune_nb(mix_nb(secret, secret*64));
    secret = prune_nb(mix_nb(secret, secret/32));
    secret = prune_nb(mix_nb(secret, secret*2048));

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
    let secret_sum = 0;
    let duration = start.elapsed();
    println!(
        "Part2: {secret_sum} | {}s {}ms {}µs {}ns",
        duration.as_secs(),
        duration.subsec_millis(),
        duration.subsec_micros() % 1000,
        duration.subsec_nanos() % 1000
    );

    Ok(())
}
