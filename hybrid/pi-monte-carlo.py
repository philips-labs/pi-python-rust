import random as r
from cffi import FFI

ffi = FFI()

ffi.cdef("""
        double montecarlo_pi(int);
""")

C = ffi.dlopen("./hybrid/pi-monte-carlo/target/release/libpi_monte_carlo_ffi.dylib")

pi_est = C.montecarlo_pi(5_000_000)
print(pi_est)
