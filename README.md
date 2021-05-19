# Monte Carlo Pi

We try to get pi using Monte Carlo.
Number of dart throwing: 5_000_000

## Benchmark tool

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

Result with sqrt:
```
Benchmark #1: python python/pi-monte-carlo.py
  Time (mean ± σ):      5.666 s ±  0.068 s    [User: 5.516 s, System: 0.111 s]
  Range (min … max):    5.584 s …  5.785 s    10 runs
```
Result without sqrt:
```
Benchmark #1: python python/pi-monte-carlo.py
  Time (mean ± σ):      5.183 s ±  0.081 s    [User: 5.042 s, System: 0.105 s]
  Range (min … max):    5.091 s …  5.337 s    10 runs
```
## Python with WASM

### Execute

### Benchmark

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

Result without sqrt:
```
Benchmark #1: ./rust/pi-monte-carlo/target/release/pi-monte-carlo
  Time (mean ± σ):      78.7 ms ±   4.5 ms    [User: 71.6 ms, System: 4.3 ms]
  Range (min … max):    73.3 ms …  92.7 ms    35 runs
```

