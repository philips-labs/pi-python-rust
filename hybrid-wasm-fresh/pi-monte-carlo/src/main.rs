extern crate rand;
use rand::Rng;
use std::env;

fn montecarlo_pi(trials: u32) -> f64 {
    let mut hit: u32 = 0;
    let mut rng = rand::thread_rng();

    for _i in 1..trials {
        let x: f64 = rng.gen::<f64>();
        let y: f64 = rng.gen::<f64>();

        // Shouldn't we do a sqrt over this?
        let position_on_board: f64 = x * x + y * y;

        if position_on_board < 1.0 {
            hit += 1;
        }
    }

    (hit as f64) * 4.0 / (trials as f64)
}

fn parse_args(args: Vec<String>) -> u32 {
    let default_trials = 5_000_000;

    if args.len() < 2 {
        return default_trials;
    } else {
        match args[1].parse() {
          Ok(n) => return n, 
          Err(_) => return default_trials
        };
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let trials = parse_args(args);
    println!("trials: {}", trials);
    let pi_est: f64 = montecarlo_pi(trials);

    println!("{pi}", pi = pi_est);
}
