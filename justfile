# Services
services-up:
    @docker compose -f compose.yaml up -d

services-down:
    @docker compose -f compose.yaml down

services-stop:
    @docker compose -f compose.yaml stop

services-restart:
    @docker compose -f compose.yaml down
    @docker compose -f compose.yaml up -d

# Migrations
migrate-add migration_name:
    @cargo sqlx migrate add {{ migration_name }}

migrate-run:
    @cargo sqlx migrate run

migrate-down:
    @cargo sqlx migrate revert

# Dev utils
dev:
    @just services-up
    @sleep 1
    @just run-watch

run:
    @cargo run

run-watch:
    @cargo watch -q -c -x run

test:
    @cargo test

test-watch:
    @cargo watch -q -c -x test

clippy:
  cargo clippy --all --all-targets --all-features

lint:
    @cargo fmt --all -- --check
    @cargo clippy --all --all-targets -- --deny warnings

lint-fix:
    @cargo fmt --all
    @cargo clippy

clean:
    @cargo clean