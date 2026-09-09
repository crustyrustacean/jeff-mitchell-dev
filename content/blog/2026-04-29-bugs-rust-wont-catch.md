+++
title = "Bugs Rust Won't Catch: Lessons from the uutils/coreutils CVE Audit"
date = 2026-04-29
description = "A professional audit of the Rust reimplementation of GNU coreutils uncovered 44 CVEs — none caught by the borrow checker, Clippy, or cargo-audit. Here's what to watch for."
categories = ["Field-Notes"]
tags = ["rust", "security", "systems-programming", "best-practices"]
draft = false
aliases = ["/2026-04-29-bugs-rust-wont-catch"]
+++

A [professional security audit of uutils/coreutils](https://corrode.dev/blog/bugs-rust-wont-catch) — the Rust reimplementation of GNU coreutils — turned up 44 CVEs. Not one was a memory-safety bug. The borrow checker, Clippy, and cargo-audit missed all of them. They all live at the boundary between Rust-controlled code and the messy outside world.

Here is what to watch for.

## 1. TOCTOU — Don't Trust a Path Across Two Syscalls

A path re-resolved between two syscalls can point to a different target — a symlink swapped in by an attacker between the two calls. Use `OpenOptions::create_new(true)` when creating files, and anchor all subsequent operations on an open file descriptor rather than re-resolving a path string.

## 2. Set Permissions at Creation, Not After

A file created with default permissions has an open window before a follow-up `chmod` closes it. Use `OpenOptions::mode()` or `DirBuilderExt::mode()` so it is born with the correct permissions. Set the umask explicitly if needed.

## 3. Resolve Paths Before Comparing Them

String comparison against `/` (or any path) can be bypassed by aliases like `/../`, `/./`, or a symlink. Always `canonicalize()` first. For two arbitrary paths, compare `(dev, inode)` pairs — not strings.

## 4. Stay in Bytes at Unix Boundaries

Rust `String`s are UTF-8; Unix paths, environment variables, and stream contents are not. `from_utf8_lossy` silently corrupts data; `unwrap`/`expect` panics on non-UTF-8 input. Use `Path`/`PathBuf` for filesystem paths, `OsString` for env vars, and `Vec<u8>` / `&[u8]` for raw streams.

## 5. Treat Every `panic!` as a Denial of Service

In code that handles untrusted input, every `unwrap`, `expect`, slice index, or unchecked cast is a potential DoS. Return proper errors with `?`, `get()`, `checked_*` arithmetic, and `try_from`. Enable Clippy lints `unwrap_used`, `expect_used`, `panic`, `indexing_slicing`, and `arithmetic_side_effects` — scoped to non-test code.

## 6. Propagate Errors — Don't Discard Them

Swallowing errors with `.ok()`, `.unwrap_or_default()`, or `let _ =` hides real failures. Track the worst exit code across a loop. If ignoring an error is intentional, explain why in a comment.

## 7. Match the Original Tool's Behavior Exactly

Behavioral divergence from the tool being reimplemented is a security risk. Shell scripts rely on exact semantics — exit codes, option parsing, edge cases. Run the upstream test suite in CI and treat bug-for-bug compatibility as a safety feature (Hyrum's Law in action).

## 8. Resolve Inputs Before Crossing a Trust Boundary

After a `chroot` (or similar boundary crossing), library calls may load code from an attacker-controlled filesystem. Resolve usernames, paths, and any external data *before* crossing the boundary, not after.

---

## What Rust Still Prevents

None of the classic memory-safety bugs appeared: no buffer overflows, use-after-free, double-free, data races, null-pointer dereferences, or uninitialized-memory reads. That foundation holds. The real security boundary in modern Rust is the interface between your code and the outside world — and that part is still up to you.

*Source: [corrode.dev — Bugs Rust Won't Catch](https://corrode.dev/blog/bugs-rust-wont-catch)*
