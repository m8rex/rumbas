#!/bin/sh
cd numbas && cargo +nightly fmt --all -- --check
cd ..
cd rumbas && cargo +nightly fmt --all -- --check
cd ..

