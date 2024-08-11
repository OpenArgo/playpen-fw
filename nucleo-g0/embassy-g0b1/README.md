# NUCELO-G0B1RE

## Quickstart
- Install `probe-rs` (see below, see section on setting up STLink probe)
- Run `rustup update && rustup target add thumbv6m-none-eabi` (Installs toolchain for Cortex M0, the architecture of the STM32G0)
- Run `cargo-embed` (Builds + flashes + Prints out RTT logs)

## Creation of Code
- Repo generated with [`cortex-m-quickstart`](https://github.com/rust-embedded/cortex-m-quickstart)
- [`stm32g0` crate](https://crates.io/crates/stm32g0)
- [`rtic`](https://rtic.rs/2/book/en/) for scheduling
    -  Note that I don't trust the embassy HAL, I'd rather decouple the HAL from processing to make code more portable if possible. (NOTE: STM32G0B1 is not supported by `stm32g0xx_hal` yet, so development stopping here for this, I don't have time to contribute to an STM HAL for this POC)

## Tooling
> Since STMs are pretty well supported, you can use whatever tooling you want for flashing and debug once you have the binary, these are just recommended

1. [`probe-rs`](https://probe.rs/docs/getting-started/installation/)
2.  `cargo`, `rustup`

