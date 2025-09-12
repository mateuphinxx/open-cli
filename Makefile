.PHONY: help build test clean dev ci demo docker-build docker-test docker-clean

help:
	@echo "OpenCLI Development Commands"
	@echo ""
	@echo "Native Commands:"
	@echo "  build       Build release binary"
	@echo "  test        Run test suite"
	@echo "  clean       Clean build artifacts"
	@echo "  dev         Start development mode"
	@echo ""
	@echo "Docker Commands:"
	@echo "  docker-build    Build Docker images"
	@echo "  docker-test     Run Docker test suite"
	@echo "  docker-ci       Run CI pipeline"
	@echo "  docker-demo     Run demo workflow"
	@echo "  docker-clean    Clean Docker resources"
	@echo ""
	@echo "Development:"
	@echo "  setup-tests     Setup test scenarios"
	@echo "  watch           Watch and auto-test"

build:
	cargo build --release

test:
	cargo test --release

clean:
	cargo clean
	rm -f opencli.log
	rm -f cache.txt

dev:
	cargo watch -x check -x test

ci:
	cargo fmt --check
	cargo clippy -- -D warnings
	cargo test --release
	cargo build --release

docker-build:
	docker-compose build

docker-test:
	chmod +x scripts/setup-test-scenarios.sh
	./scripts/setup-test-scenarios.sh
	docker-compose -f docker-compose.test.yml up --abort-on-container-exit

docker-ci:
	docker-compose -f docker-compose.ci.yml up --abort-on-container-exit

docker-demo:
	docker-compose up demo

docker-clean:
	docker-compose down -v
	docker-compose -f docker-compose.ci.yml down -v
	docker-compose -f docker-compose.test.yml down -v
	docker system prune -f

setup-tests:
	chmod +x scripts/setup-test-scenarios.sh
	./scripts/setup-test-scenarios.sh

watch:
	docker-compose run --rm dev cargo watch -x check -x test

shell:
	docker-compose run --rm dev bash

release: clean build
	strip target/release/opencli
	cp target/release/opencli ./opencli
