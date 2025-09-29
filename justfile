import 'docker.just'
import? 'private.just'

image_name := "ghcr.io/rezi-labs/rezi-web"
export NEST_API_KEY := "hello_world"
export LOCAL :="true"

docker: db
    docker compose up

it:
    cargo install cargo-watch --locked
    curl -sSfL https://get.tur.so/install.sh | bash

run: db
    cargo run

db:
    -(kill -9 $(lsof -t -i:8080))
    turso dev &

watch:
    cargo watch -x run

verify: lint test

test:
    cargo test

lint:
    cargo fmt --all -- --check
    cargo clippy

fmt:
    cargo fmt
    cargo fix --allow-dirty --allow-staged

generate-session-secret:
    openssl rand -base64 64
