# Build and Verification Commands

Reference for building, testing, and verifying HAMR component crates targeting seL4 Microkit with Rust.

## Crate Layout

Component crates are in `hamr/microkit/crates/`. Each HAMR thread component has a crate named `<process>_<thread>` (e.g., `gate_gate`, `msg_filter_msg_filter`).

Two supporting crates are also present and should be excluded from component-level operations:
- **`data`** -- Shared data type definitions (auto-generated)
- **`GumboLib`** -- GUMBO library for formal contract specifications

## Per-Crate Makefile Targets

Run these from within a component crate directory (e.g., `crates/gate_gate/`):

| Target | Command | Purpose |
|--------|---------|---------|
| `make verus` | `cargo-verus verify` | Run Verus verification only (no build artifacts) |
| `make verus-json` | `cargo-verus verify -- --output-json` | Verus verification with JSON output to `verus_results.json` |
| `make build-verus-release` | `cargo-verus build --release` | Build with Verus verification (production release) |
| `make build-verus` | `cargo-verus build` | Build with Verus verification (debug) |
| `make build-release` | `cargo build --release` | Build without Verus verification (production release) |
| `make build` | `cargo build` | Build without Verus verification (debug) |
| `make test` | `cargo test` | Run unit tests (pass `args=<filter>` to filter, e.g., `make test args=proptest`) |
| `make test-release` | `cargo test --release` | Run unit tests in release mode |
| `make coverage` | `grcov` + `cargo test` | Generate HTML test coverage report |
| `make clean` | `cargo clean` | Clean build artifacts |

### Notes

- `make verus` is the fastest way to check if code passes Verus verification -- it verifies without producing build artifacts.
- `make all` defaults to `build-verus-release` (builds with verification enabled).
- To skip verification during builds, use `make build-release` or `make build` instead.
- Tests run without seL4 libraries -- the infrastructure uses `Mutex`-guarded statics instead of shared memory in test mode.

## Top-Level Makefile Targets

Run these from `hamr/microkit/`:

| Target | Purpose |
|--------|---------|
| `make all` | Build the full system image (requires `MICROKIT_SDK` and `MICROKIT_BOARD` environment variables) |
| `make test` | Run tests for all component crates |
| `make verus` | Run Verus verification for all component crates |
| `make qemu` | Run the system image on QEMU simulator |
| `make clean` | Clean build artifacts |
| `make clobber` | Clean and remove the entire build directory |

The top-level `make all` uses `cargo-verus` by default, so verification must pass for the build to succeed. To skip verification: `RUST_MAKE_TARGET=build-release make`.

## Environment

- **`cargo-verus`** -- The Verus-enabled Cargo wrapper. Must be on `PATH`.
- **`RUSTC_BOOTSTRAP=1`** -- Set automatically by the Makefiles (required for Verus build-std flags).
- **Cross-compilation target**: `aarch64-unknown-none` (set automatically by `CARGO_FLAGS` in each Makefile).
- **`MICROKIT_SDK`** and **`MICROKIT_BOARD`** -- Required for top-level builds and QEMU but NOT required for `make verus` or `make test` in individual crates.
