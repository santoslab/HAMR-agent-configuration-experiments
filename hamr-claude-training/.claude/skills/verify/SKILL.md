---
name: verify
description: Run Verus verification on all HAMR component crates and summarize results
---

# Verify All HAMR Component Crates

Run Verus verification on every component crate in the project and provide a summary of results.

## Steps

1. Find the `hamr/microkit/crates/` directory in the project. List its contents.

2. Identify **component crates** -- exclude `data` and `GumboLib` (these are supporting crates, not thread components).

3. For each component crate, run `make verus` from that crate's directory. Capture both stdout and stderr. Do NOT stop if one crate fails -- continue to verify all crates.

4. After running all crates, produce a summary table:

| Crate | Result | Details |
|-------|--------|---------|
| crate_name | PASS / FAIL | "N verified, M errors" or error category |

5. If there are failures, group them by error category (e.g., "external function used in verus block", "arithmetic overflow", "postcondition not satisfied") and list the affected file and line for each.

## Important

- Run crates sequentially (parallel `cargo-verus` invocations may conflict on shared build state).
- The `make verus` target only runs verification -- it does not produce build artifacts.
- `MICROKIT_SDK` and `MICROKIT_BOARD` are NOT required for `make verus`.
