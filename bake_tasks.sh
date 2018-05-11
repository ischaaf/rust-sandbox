#!/usr/bin/env bash

function build () {
  cargo build "$@"
}

function run () {
  cargo run "$@"
}
