import wasmtime.loader

import pi_monte_carlo

pi_est = pi_monte_carlo.montecarlo_pi(5_000_000)
print(pi_est)
