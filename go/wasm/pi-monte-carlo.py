import wasmtime.loader

import main

pi_est = instance.exports.monteCarloPi(5_000_000)
print(pi_est)
