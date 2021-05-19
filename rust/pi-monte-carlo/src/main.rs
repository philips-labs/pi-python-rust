extern crate rand;

use rand::Rng;

fn montecarlo_pi(trials: u32) -> f64 {
    let mut count: u32 = 0;
    let mut rng = rand::thread_rng();

    for _i in 1..trials {
        let x: f64 = rng.gen::<f64>();
        let y: f64 = rng.gen::<f64>();
        let p: f64 = x * x + y * y;
        
        if p < 1.0 {
            count += 1;
        }
    }

    let pi_est: f64 = (count as f64) * 4.0 / (trials as f64);
    pi_est
}

fn main() {
    let pi_est: f64 = montecarlo_pi(5_000_000);
    
    println!("[1] {pi}", pi = pi_est);
}
