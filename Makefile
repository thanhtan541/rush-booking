# cargo install cargo-watch
dev:
	cargo watch -x check -x test -x run

fmt:
	cargo fmt

check: fmt
	cargo check

PATTERN?="update_db"
test: check
	cargo test ${PATTERN}

test-verbose: check
	cargo test -- --nocapture

# cargo install cargo-tarpaulin
cov:
	cargo tarpaulin --ignore-tests

# rustup component add clippy
lint-check: check
	cargo clippy -- -D warnings

# rustup component add rustfmt
fmt-check:
	cargo fmt -- --check

# cargo install cargo-audit
audit:
	cargo audit

# cargo install cargo-deny
# equivalent to cargo-audit
deny-audit:
	cargo deny

build:
	cargo build

# cargo install cargo-asm
asm:
	cargo asm

# cargo install bunyan
test-log:
	export RUST_LOG="sqlx=error,info"
	export TEST_LOG=true
	cargo test ${PATTERN} | bunyan

# slqx cli is requied
# cargo install --version="~0.7" sqlx-cli --no-default-features \
  --features rustls,postgres
# Expose databse url for local developement
# export DATABASE_URL=postgres://postgres:password@127.0.0.1:5432/rush_booking

init-db:
	./scripts/init_db.sh

init-sqlx-offline:
	export DATABASE_URL=postgres://postgres:password@127.0.0.1:5432/hotel_booking
	cargo sqlx prepare --workspace

MIGRATION?="update_db"
add-migration:
	sqlx migrate add $(MIGRATION)

# Required to setup docker first
run-migration:
	SKIP_DOCKER=true ./scripts/init_db.sh

# cargo install cargo-udeps
scan:
	cargo +nightly udeps

docker-build:
	docker build --tag zero2prod --file Dockerfile .

docker-run:
	docker run -p 8000:8000 zero2prod

# Install Digital ocean's cli
# brew install doctl
cloud-init:
	doctl apps create --spec spec.yaml

cloud-validate:
	doctl apps spec validate spec.yaml

cloud-auth:
	doctl auth init

cloud-apps:
	doctl apps list

cloud-apps-db-migrate:
	DATABASE_URL=postgresql://newsletter:AVNS_VaVs7648xubCMx8igZZ@app-c862bb64-6968-4be2-b722-b72609150f7b-do-user-16862819-0.c.db.ondigitalocean.com:25060/newsletter?sslmode=require sqlx migrate run

# JWT
jwt-keypair:
	./scripts/init_jwt_keypair.sh
