# Monte Carlo Pi

## Purpose of this repository

We want to show how you can improve performance of your application by using WASM. 
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
- [ ] [Python calling Rust with wasm] version in [/hybrid-wasm](/hybrid-wasm).

We're using the [hyperfine](https://github.com/sharkdp/hyperfine) benchmark tool.

```bash
brew install hyperfine
```


## Python

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
  Time (mean ± σ):      5.183 s ±  0.081 s    [User: 5.042 s, System: 0.105 s]
  Range (min … max):    5.091 s …  5.337 s    10 runs
```

## Rust

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
  Time (mean ± σ):      78.7 ms ±   4.5 ms    [User: 71.6 ms, System: 4.3 ms]
  Range (min … max):    73.3 ms …  92.7 ms    35 runs
```

## Hybrid

### Execute

```bash
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
  Time (mean ± σ):     300.6 ms ±   9.7 ms    [User: 170.2 ms, System: 109.2 ms]
  Range (min … max):   289.1 ms … 314.5 ms    10 runs


