alias c := clean
alias r := run

default:
    @just --list

clean:
    rm -f *.ppm

all:
    cargo run --release -- first-ppm
    cargo run --release -- gradient
    cargo run --release -- ray-sphere
    cargo run --release -- ray-sphere-normal
    cargo run --release -- hittables
    cargo run --release -- anti-aliasing
    cargo run --release -- first-diffuse
    cargo run --release -- diffuse-no-acne
    cargo run --release -- lambertian
    cargo run --release -- gamma
    cargo run --release -- metal

run THING:
    cargo run --release -- {{THING}}
