#!/usr/bin/env python3
"""
Generate Argon2id password hashes compatible with the Rust backend.

Usage:
    ./scripts/hash_password.py
    ./scripts/hash_password.py --help

Requires:
    pip install --user argon2-cffi

Parameters match `Argon2::default()` from the Rust `argon2` crate v0.6:
    algorithm   : Argon2id
    version     : 0x13 (v=19)
    memory_cost : 19456 KiB (~19 MiB)
    time_cost   : 2
    parallelism : 1
    hash_len    : 32 bytes
    salt_len    : 16 bytes
"""

import argparse
import getpass
import sys

try:
    from argon2 import PasswordHasher
    from argon2.exceptions import HashingError, VerifyMismatchError
except ImportError:
    print("ERROR: 'argon2-cffi' is not installed.", file=sys.stderr)
    print("Install with: pip install --user argon2-cffi", file=sys.stderr)
    sys.exit(1)


def build_hasher() -> PasswordHasher:
    """Build a PasswordHasher with parameters matching the Rust backend."""
    return PasswordHasher(
        time_cost=2,
        memory_cost=19456,
        parallelism=1,
        hash_len=32,
        salt_len=16,
    )


def prompt_password() -> str:
    """Prompt for password + confirmation. Repeats until both match."""
    while True:
        p1 = getpass.getpass("Password        : ")
        if not p1:
            print("  ! password cannot be empty", file=sys.stderr)
            continue
        p2 = getpass.getpass("Confirm password: ")
        if p1 != p2:
            print("  ! passwords do not match, try again", file=sys.stderr)
            continue
        return p1


def main() -> int:
    parser = argparse.ArgumentParser(
        description="Generate an Argon2id password hash for the Galonku database.",
    )
    parser.add_argument(
        "--password",
        help="Password to hash (skips interactive prompt). "
             "WARNING: the password may be stored in shell history.",
    )
    parser.add_argument(
        "--no-verify",
        action="store_true",
        help="Skip the self-verification step (not recommended).",
    )
    args = parser.parse_args()

    hasher = build_hasher()

    if args.password is not None:
        password = args.password
        if not password:
            print("ERROR: password cannot be empty", file=sys.stderr)
            return 1
    else:
        print("=== Argon2id Password Hash Generator ===")
        print("Params: m=19456, t=2, p=1 (matches Rust `Argon2::default()`)")
        print()
        password = prompt_password()

    try:
        hash_str = hasher.hash(password)
    except HashingError as exc:
        print(f"ERROR: hashing failed: {exc}", file=sys.stderr)
        return 1

    if not args.no_verify:
        try:
            hasher.verify(hash_str, password)
        except VerifyMismatchError:
            print("ERROR: self-verification failed (bug!)", file=sys.stderr)
            return 1

    print()
    print("=== Generated hash ===")
    print(hash_str)
    print()
    print("Copy the line above into your SQL seeder.")
    return 0


if __name__ == "__main__":
    sys.exit(main())
