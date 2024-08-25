alias c := clean

default:
    @just --list

clean:
    rm -f *.ppm

all:
    cargo run -- first-ppm
    cargo run -- gradient
    cargo run -- ray-sphere
    cargo run -- ray-sphere-normal
