#!/usr/bin/env python3
import subprocess
import sys

def run_command(command, description, success_msg, error_msg, fix_hint=None):
    """
    Menjalankan perintah shell dan menampilkan hasilnya.
    Jika perintah gagal, tampilkan pesan error dan keluar dari program.
    """
    print(description)
    try:
        # Jalankan perintaj shell
        subprocess.run(command, shell=True, check=True)
        print(success_msg)
    except subprocess.CalledProcessError:
        # Jika gagal, tampilkan pesan error dan saran perbaikan (jika ada)
        print(error_msg)
        if fix_hint:
            print()
            print("Jalankan perintah ini untuk memperbaiki:")
            print(f"  {fix_hint}")
        sys.exit(1)
    print()

def main():
    print()
    print("========================================")
    print("Memeriksa Kualitas Kode")
    print("========================================")
    print()

    # [1/3] Periksa format kode
    run_command(
        "cargo fmt --all -- --check",
        "[1/3] Memeriksa format kode...",
        "[OK] Format kode sudah benar",
        "[ERROR] Format kode belum benar",
        "cargo fmt --all\n  atau: ./scripts/format.sh"
    )

    # [2/3] Jalankan linter Clippy
    run_command(
        "cargo clippy --all-targets --all-features -- -D warnings",
        "[2/3] Menjalankan Clippy linter...",
        "[OK] Tidak ada peringatan dari Clippy",
        "[ERROR] Ditemukan masalah oleh Clippy"
    )

    # [3/3] Periksa kompilasi kode
    run_command(
        "cargo check --all-targets --all-features",
        "[3/3] Memeriksa kompilasi kode...",
        "[OK] Kompilasi berhasil",
        "[ERROR] Gagal dalam pemeriksaan kompilasi"
    )

    print("========================================")
    print("[SUKSES] Semua pemeriksaan berhasil!")
    print("========================================")
    print()

if __name__ == "__main__":
    try:
        # Jalankan fungsi utama
        main()
    except KeyboardInterrupt:
        # Tangani interupsi pengguna (Ctrl + C)
        print("\n[DIBATALKAN] Dihentikan oleh pengguna")
        sys.exit(1)
