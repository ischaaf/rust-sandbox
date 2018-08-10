#!/usr/bin/env bash

function build () {
  cargo build "$@"
}

function run () {
  cargo run "$@"
}

function run.directly () {
  ./target/debug/rust-sandbox "$@"
}
