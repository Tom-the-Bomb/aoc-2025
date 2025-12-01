@ECHO off

cargo clippy --workspace --all-features -- -D clippy::all -D clippy::pedantic -D clippy::nursery -D clippy::cargo