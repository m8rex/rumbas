# Rumbas 

## What is this folder ?

- This folder contains the source code for the rumbas binary
- It uses the numbas crate (see numbas folder) to interact with the numbas exam files

## Installing

- Make sure python 3 and rust are installed (and added to the path)
- Build with `cargo build --release`
- Install with `cargo install --path .`
- Clone numbas from https://github.com/numbas/Numbas
- Run `NUMBAS_FOLDER=<path to numbas> rumbas <relative path to exam file>` in a rumbas folder
  - Example: `NUMBAS_FOLDER=/Programming/Numbas rumbas exams/rumbas-exam-test.json` in `examples/simple-example`

## Building

- Build with `cargo build --release`
