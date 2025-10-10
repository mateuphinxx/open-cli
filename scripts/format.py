#!/usr/bin/env python3
import subprocess
import sys

def main():
    print("Formatting Rust code...")
    try:
        subprocess.run("cargo fmt --all", shell=True, check=True)
    except subprocess.CalledProcessError:
        print("[ERROR] Failed to format code.")
        sys.exit(1)

    print("[SUCCESS] Code formatted successfully!")
    print()
    print("Run this to check formatting:")
    print("  cargo fmt --all -- --check")

if __name__ == "__main__":
    try:
        main()
    except KeyboardInterrupt:
        print("\n[ABORTED] User interrupted")
        sys.exit(1)

