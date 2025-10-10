#!/usr/bin/env python3
import subprocess
import sys

def main():
    print()
    print("========================================")
    print("Menjalankan Pengujian")
    print("========================================")
    print()

    # Menjalankan unit test menggunakan Cargo
    print("Menjalankan unit test...")
    try:
        # Menjalankan perintah cargo test dengan mode rilis dan output detail
        subprocess.run("cargo test --release --verbose", shell=True, check=True)
    except subprocess.CalledProcessError:
        # Jika ada error saat menjalankan test, tampilkan pesan error
        print()
        print("========================================")
        print("[ERROR] Pengujian gagal!")
        print("========================================")
        sys.exit(1)

    # Jika semua test berhasil
    print()
    print("========================================")
    print("[SUKSES] Semua pengujian berhasil!")
    print("========================================")

if __name__ == "__main__":
    try:
        # Menjalankan fungsi utama
        main()
    except KeyboardInterrupt:
        # Menangani interupsi dari pengguna (Ctrl + C)
        print("\n[DIBATALKAN] Dihentikan oleh pengguna")
        sys.exit(1)
