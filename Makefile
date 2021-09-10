build:
	rustup component add rustfmt
	cargo build --release

docker-build:
	docker run --rm -v "$(shell pwd)":/usr/src/addcan rust:1.54.0-slim-buster /bin/bash -c "cd /usr/src/addcan && apt update && apt install make -y && make"