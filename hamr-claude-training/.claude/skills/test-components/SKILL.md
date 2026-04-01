---
name: test-components
description: Run tests on all HAMR component crates and summarize results
---

# Test All HAMR Component Crates

Run unit tests on every component crate in the project and provide a summary of results.

## Steps

1. Find the `hamr/microkit/crates/` directory in the project. List its contents.

2. Identify **component crates** -- exclude `data` and `GumboLib` (these are supporting crates, not thread components).

3. For each component crate, run `make test` from that crate's directory. Capture both stdout and stderr. Do NOT stop if one crate fails -- continue to test all crates.

4. After running all crates, produce a summary table:

| Crate | Result | Details |
|-------|--------|---------|
| crate_name | PASS / FAIL | "N tests passed, M failed" or error details |

5. If there are failures, show the relevant test output for each failing test (test name, assertion message, and location).

## Important

- Tests run without seL4 libraries -- the test infrastructure uses `Mutex`-guarded statics instead of shared memory.
- `MICROKIT_SDK` and `MICROKIT_BOARD` are NOT required for `make test`.
- To run only specific tests, developers can use `make test args=<filter>` directly (e.g., `make test args=proptest`).
