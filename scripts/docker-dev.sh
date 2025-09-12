#!/bin/bash

set -e

case "$1" in
    "dev")
        docker compose up dev
        ;;
    "build")
        docker compose up build
        ;;
    "test")
        docker compose -f docker-compose.test.yml up --abort-on-container-exit
        ;;
    "ci")
        docker compose -f docker-compose.ci.yml up --abort-on-container-exit
        ;;
    "demo")
        docker compose up demo
        ;;
    "clean")
        docker compose down -v
        docker system prune -f
        ;;
    "shell")
        docker compose run --rm dev bash
        ;;
    "watch")
        docker compose run --rm dev cargo watch -x check -x test
        ;;
    *)
        echo "Usage: $0 {dev|build|test|ci|demo|clean|shell|watch}"
        echo ""
        echo "Commands:"
        echo "  dev    - Start development environment"
        echo "  build  - Build release binary"
        echo "  test   - Run test suite"
        echo "  ci     - Run CI pipeline"
        echo "  demo   - Run demo workflow"
        echo "  clean  - Clean up containers and volumes"
        echo "  shell  - Open development shell"
        echo "  watch  - Watch for changes and auto-test"
        exit 1
        ;;
esac
