# NUCELO-G0B1RE

## Quickstart
- Install `probe-rs` (see below, see section on setting up STLink probe)
- Run `rustup update && rustup target add thumbv6m-none-eabi` (Installs toolchain for Cortex M0, the architecture of the STM32G0)
- Run `cargo-embed` (Builds + flashes + Prints out RTT logs)

## Creation of Code
- Repo heavily based on [Embassy STM32G0 blinky example](https://github.com/embassy-rs/embassy/blob/fdc34b69ffc292427f50f35209f095bb5a50bb82/examples/stm32g0/src/bin/blinky.rs)

## Tooling
> Since STMs are pretty well supported, you can use whatever tooling you want for flashing and debug once you have the binary, these are just recommended

1. [`probe-rs`](https://probe.rs/docs/getting-started/installation/)
2.  `cargo`, `rustup`

