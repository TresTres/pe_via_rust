# PE Suite via Rust

This is a simple suite to capture Rust practice through the first 100 problems in [Project Euler](https://projecteuler.net/about).

## Get Rust

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

## Run a solution

Quick run
```bash
rustc src/main.rs
./main <problem number> 
```

If using cargo, you can run the solution with the following command:

```bash
cargo build
cargo run <problem number>
```

## Run tests

```bash
cargo test
```


## Contribution
Just as a reminder, try not to publish solutions beyond the first 100.

To add a new solution:
1. Locate the pro
2. Create a new file in `pe_via_rust/src/anthology` with the name `solution<problem number>.rs` and implement the `process` function, which should return a `String`.
3. Add the solution to `src/anthology.rs` with an entry of 
```rust
    directory.insert(
    6,
    (SolutionInfo { 
        index: 6, problem_name: String::from("Sum Square Difference") },
        solution6::process
    )
);