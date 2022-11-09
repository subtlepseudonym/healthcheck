BINARY=healthcheck
BUILD=$$(vtag --no-meta)
TAG="subtlepseudonym/${BINARY}:${BUILD}"

default: build

build:
	cargo build --release

docker:
	docker build --tag ${TAG} -f Dockerfile .

get-tag:
	echo ${BUILD}

.PHONY: all build docker get-tag
