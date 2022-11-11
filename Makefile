BINARY=healthcheck
BUILD=$$(vtag --no-meta)
TAG="subtlepseudonym/${BINARY}:${BUILD}"

default: build

build:
	cargo build

release:
	 cargo +nightly build \
		 -Z build-std=std,panic_abort \
		 -Z build-std-features=panic_immediate_abort \
		 --target x86_64-unknown-linux-musl \
		 --release

docker:
	docker build --tag ${TAG} -f Dockerfile .

get-tag:
	echo ${BUILD}

.PHONY: all build docker get-tag
