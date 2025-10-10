#!/usr/bin/env python3
import subprocess
import sys

def main():
    print()
    print("========================================")
    print("Running Tests")
    print("========================================")
    print()

    print("Running unit tests...")
    try:
        subprocess.run("cargo test --release --verbose", shell=True, check=True)
    except subprocess.CalledProcessError:
        print()
        print("========================================")
        print("[ERROR] Tests failed!")
        print("========================================")
        sys.exit(1)

    print()
    print("========================================")
    print("[SUCCESS] All tests passed!")
    print("========================================")

if __name__ == "__main__":
    try:
        main()
    except KeyboardInterrupt:
        print("\n[ABORTED] User interrupted")
        sys.exit(1)

