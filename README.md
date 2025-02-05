# Tramer

[![Crates.io](https://img.shields.io/crates/v/tramer.svg)](https://crates.io/crates/tramer)
[![Documentation](https://docs.rs/tramer/badge.svg)](https://docs.rs/tramer)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/yourusername/tramer/blob/main/LICENSE)

## Usage
> add `tramer` to your `Cargo.toml`:
```toml
[dependencies]
tramer = "0.1.0"
```

> use it:
```rs
use tramer::tramer;

#[tramer("millis")]
fn profile_millis() {
    std::thread::sleep(std::time::Duration::from_millis(150));
}

#[tramer("nanos")]
fn profile_nanos() {
    std::thread::sleep(std::time::Duration::from_micros(300));
}

#[tramer("secs")]
fn profile_seconds() {
    std::thread::sleep(std::time::Duration::from_millis(250));
}
```
