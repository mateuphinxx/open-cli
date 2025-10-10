#!/usr/bin/env python3
import subprocess
import sys

def run_command(command, description, success_msg, error_msg, fix_hint=None):
    print(description)
    try:
        subprocess.run(command, shell=True, check=True)
        print(success_msg)
    except subprocess.CalledProcessError:
        print(error_msg)
        if fix_hint:
            print()
            print("Run this to fix:")
            print(f"  {fix_hint}")
        sys.exit(1)
    print()

def main():
    print()
    print("========================================")
    print("Checking Code Quality")
    print("========================================")
    print()

    # [1/3] Checking code formatting
    run_command(
        "cargo fmt --all -- --check",
        "[1/3] Checking code formatting...",
        "[OK] Code formatting is correct",
        "[ERROR] Code is not formatted correctly",
        "cargo fmt --all\n  or: ./scripts/format.sh"
    )

    # [2/3] Running Clippy linter
    run_command(
        "cargo clippy --all-targets --all-features -- -D warnings",
        "[2/3] Running Clippy linter...",
        "[OK] No Clippy warnings",
        "[ERROR] Clippy found issues"
    )

    # [3/3] Checking compilation
    run_command(
        "cargo check --all-targets --all-features",
        "[3/3] Checking compilation...",
        "[OK] Code compiles successfully",
        "[ERROR] Compilation check failed"
    )

    print("========================================")
    print("[SUCCESS] All checks passed!")
    print("========================================")
    print()

if __name__ == "__main__":
    # Exit immediately on failure (like `set -e`)
    try:
        main()
    except KeyboardInterrupt:
        print("\n[ABORTED] User interrupted")
        sys.exit(1)

