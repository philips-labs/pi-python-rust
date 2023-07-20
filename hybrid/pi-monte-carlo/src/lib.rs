extern crate rand;

use rand::Rng;

#[no_mangle]
pub fn montecarlo_pi(trials: u32) -> f64 {
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
