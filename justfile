import 'docker.just'

image_name := "ghcr.io/lunchtimecode/grocy"
export NEST_API_KEY := "hello_world"
export OPEN_BROWSER := "true"

docker: db
    docker compose up

it:
    cargo install cargo-watch --locked
    curl -sSfL https://get.tur.so/install.sh | bash
    sudo curl -o /etc/yum.repos.d/beekeeper-studio.repo https://rpm.beekeeperstudio.io/beekeeper-studio.repo
    sudo rpm --import https://rpm.beekeeperstudio.io/beekeeper.key
    sudo dnf install beekeeper-studio

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
