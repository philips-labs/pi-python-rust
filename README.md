# Monte Carlo Pi

## Purpose of this repository

We want to show how you can improve performance of your application by using WASM/WASI. 
To show this, we're using an [estimation](https://www.youtube.com/watch?v=MhbT7EvYN0c) algorithm to calculate pi.

By implementing this in different languages and benchmarking it, we can see if you can achieve some speed improvements.

> This repository is not here to bash Python, there are other repositories, blogs which do this a lot better. This repository is here to show how to gradually improve your existing codebase.

### Pi
Pi is a very interesting number. Although you can find the digit easily in libraries or in [documentation](http://www.geom.uiuc.edu/~huberty/math5337/groupe/digits.html), we still want to calculate it ourselves.

We want to [estimate pi with Monte Carlo](https://academo.org/demos/estimating-pi-monte-carlo/) by throwing f.e. 5_000_000 (virtual) darts at a board. The number of hits inside the board can be used to calculate pi.

Number of [dart throwing](https://www.youtube.com/watch?v=6nhgLmzjgXM): 5_000_000


## Implementations

We're using four implementations of the algorithm:
- [x] [Python](https://www.python.org/) version in [/python](/python).
- [x] [Rust](https://www.rust-lang.org/) version in [/rust](/rust).
- [x] [Python calling Rust with cffi](https://bheisler.github.io/post/calling-rust-in-python/) version in [/hybrid](/hybrid).
- [x] Python calling Rust with [WASM/WASI](https://wasi.dev/) version in [/wasm](/wasm).

We're using the [hyperfine](https://github.com/sharkdp/hyperfine) benchmark tool.

```bash
brew install hyperfine
```

## Python

Run pi-monte-carlo algorithm in pure Python!
<details>
  <summary>Click to expand!</summary>

  ### Execute

  ```bash
  python python/pi-monte-carlo.py
  ```

  ### Benchmark

  Commando:
  ``` bash
  hyperfine -w 2 -m 10 'python python/pi-monte-carlo.py'
  ```

  Result (Ran on my macbook pro):
  ```
  Benchmark #1: python python/pi-monte-carlo.py
    Time (mean ± σ):      3.184 s ±  0.072 s    [User: 3.083 s, System: 0.077 s]
    Range (min … max):    3.090 s …  3.314 s    10 runs
  ```
</details>

## Rust

Run pi-monte-carlo algorithm in pure Rust!

<details>
  <summary>Click to expand!</summary>

   ### Execute

  ```bash
  cargo run --manifest-path rust/pi-monte-carlo/Cargo.toml
  ```

  ### Benchmark

  Commando:
  ``` bash
  cargo build --release --manifest-path rust/pi-monte-carlo/Cargo.toml
  hyperfine -w 2 -m 10 './rust/pi-monte-carlo/target/release/pi-monte-carlo'
  ```

  Result (Ran on my macbook pro):
  ```
  Benchmark #1: ./rust/pi-monte-carlo/target/release/pi-monte-carlo
    Time (mean ± σ):      64.6 ms ±   1.5 ms    [User: 60.4 ms, System: 2.7 ms]
    Range (min … max):    63.4 ms …  72.3 ms    41 runs
  ```
</details>

## Hybrid

Use FFI to call the Rust monte-carlo-pi loop from Python.

<details>
  <summary>Click to expand!</summary>

  ### Execute

  ```bash
  pip install cffi
  cargo build --release --manifest-path hybrid/pi-monte-carlo/Cargo.toml
  python hybrid/pi-monte-carlo.py
  ```

  ### Benchmark

  Commando:
  ``` bash
  cargo build --release --manifest-path hybrid/pi-monte-carlo/Cargo.toml
  hyperfine -w 2 -m 10 'python hybrid/pi-monte-carlo.py'
  ```

  Result (Ran on my macbook pro):
  ```
  Benchmark #1: python hybrid/pi-monte-carlo.py
    Time (mean ± σ):     226.1 ms ±   1.7 ms    [User: 139.4 ms, System: 81.8 ms]
    Range (min … max):   222.5 ms … 229.2 ms    12 runs
  ```

  ### Advantage

  Fast

  ### Disadvantage

  Target specific libs. For each OS you have a different binary. For mac for a `.dylib` file. For windows a `.dll`.
</details>

## WASM/WASI

Using WASM/WASI to call the Rust monte-carlo-pi loop from Python.

<details>
  <summary>Click to expand!</summary>

  ### Execute

  ```bash
  cargo install wasm-gc
  pip install wasmtime
  
  cargo build --target wasm32-wasi --release --manifest-path wasm/pi-monte-carlo/Cargo.toml
  ln -s pi-monte-carlo/target/wasm32-wasi/release/pi_monte_carlo.wasm wasm/pi_monte_carlo.wasm
  wasm-gc wasm/pi-monte-carlo/target/wasm32-wasi/release/pi_monte_carlo.wasm
  python wasm/pi-monte-carlo.py
  ```

  ### Benchmark

  Commando:
  ``` bash
  cargo build --target wasm32-wasi --release --manifest-path wasm/pi-monte-carlo/Cargo.toml
  ln -s wasm/pi-monte-carlo/target/wasm32-wasi/release/pi_monte_carlo.wasm wasm
  hyperfine -w 2 -m 10 'python wasm/pi-monte-carlo.py'
  ```

  Result (Ran on my macbook pro):
  ```
  Benchmark #1: python wasm/pi-monte-carlo.py
    Time (mean ± σ):      1.551 s ±  0.069 s    [User: 1.454 s, System: 0.087 s]
    Range (min … max):    1.479 s …  1.715 s    10 runs
  ```

  ### Advantage
  - No platform specific binaries.
  - Sandboxed environment.
  - Can be called from almost any language / platform.

  ### Disadvantage

  - Not so fast

</details>



