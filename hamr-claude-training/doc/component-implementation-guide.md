# Component Implementation Guide

How to implement Rust application logic for HAMR thread components targeting seL4 Microkit.

## Overview

After HAMR code generation, the developer implements application logic in two key files per component:

- **`crates/<component>/src/component/<component>_app.rs`** -- Application logic (entry points)
- **`crates/<component>/src/test/tests.rs`** -- Unit and property-based tests (see @testing-guide.md)

Component crate names follow the convention `<process_part>_<thread_part>`. For example, a thread `gate` inside process `gate` produces crate `gate_gate`.

## Verus Blocks

All application code goes inside `verus! { }` blocks. Verus is a verification tool for Rust that checks code against formal contracts:

```rust
use data::*;
use crate::bridge::my_component_api::*;
use vstd::prelude::*;

verus! {
    pub struct my_component {
        // state variables
    }

    impl my_component {
        // entry points go here
    }

    // helper functions go here
}
```

## Component Structure

### State Struct and Constructor

The component struct holds local state. GUMBO-declared state variables appear in `BEGIN MARKER STATE VARS` / `END MARKER STATE VARS` regions (auto-managed by HAMR). Additional developer state can be added outside these regions.

```rust
pub struct my_component {
    // BEGIN MARKER STATE VARS
    // (HAMR auto-generates GUMBO state variables here)
    // END MARKER STATE VARS
}

pub fn new() -> Self {
    Self {
        // BEGIN MARKER STATE VAR INIT
        // (HAMR auto-generates initial values here)
        // END MARKER STATE VAR INIT
    }
}
```

### Initialize Entry Point

Called once during system initialization. Use it to set initial component state and initialize output ports. **DataPorts must be initialized; EventDataPorts do not need initialization.**

```rust
pub fn initialize<API: my_component_Put_Api>(
    &mut self,
    api: &mut my_component_Application_Api<API>)
    ensures
        // HAMR-generated GUMBO ensures clauses
{
    log_info("initialize entrypoint invoked");
    // Initialize DataPort outputs here (required):
    //   api.put_data_output(initial_value);
    // EventDataPort outputs do NOT need initialization
}
```

The initialize entry point receives a `Put_Api` (output only -- no input port access).

### Compute Entry Point (Periodic)

For periodic threads, the compute entry point is `timeTriggered`, called at each scheduling period:

```rust
pub fn timeTriggered<API: my_component_Full_Api>(
    &mut self,
    api: &mut my_component_Application_Api<API>)
    requires
        // HAMR-generated GUMBO requires clauses
    ensures
        // HAMR-generated GUMBO ensures clauses
{
    // Read inputs, compute, write outputs
}
```

The compute entry point receives a `Full_Api` (both input and output access).

### Compute Entry Point (Sporadic)

For sporadic threads, the compute entry point is a handler method named after the triggering event port.

### Notify Method

Handles unrecognized Microkit channel notifications. Typically just logs a warning:

```rust
pub fn notify(&mut self, channel: microkit_channel) {
    match channel {
        _ => { log_warn_channel(channel) }
    }
}
```

## Port API Usage

### Reading Input Ports

For **EventDataPort** inputs, `get_<port>()` returns `Option<T>`:

```rust
let input = api.get_input();  // Option<Message>
match input {
    Some(msg) => {
        // process the message
    }
    None => {
        // no message present
    }
};
```

For **DataPort** inputs, `get_<port>()` returns the value directly (always present after initialization).

### Writing Output Ports

For all port types, `put_<port>(value)` sends a value:

```rust
api.put_output(msg);           // send a message
api.put_data_port(value);      // set a data port value
```

## GUMBO-to-Verus Contract Mapping

GUMBO integration constraints in SysMLv2 models are translated to Verus contracts on the port API methods:

| GUMBO Constraint | Verus Contract | Location |
|---|---|---|
| Integration `guarantee` on output port | `requires` clause on `put_<port>()` | Caller must satisfy before sending |
| Integration `assume` on input port | `ensures` clause on `get_<port>()` | API guarantees after reading |

These contracts are checked at compile time by Verus and at runtime by the GUMBOX test infrastructure.

## Logging

Logging functions must be marked `#[verifier::external_body]` because they contain string formatting that Verus cannot verify:

```rust
#[verifier::external_body]
pub fn log_info(msg: &str) {
    log::info!("{0}", msg);
}

#[verifier::external_body]
pub fn log_message_dropped(msg: SNG_Data_Model::Message) {
    log::info!("DROPPED message (security_level={0:?}, payload={1})",
        msg.security_level, msg.payload);
}
```

Use `{0}`, `{1}`, etc. positional format specifiers. Use `{0:?}` for debug formatting (e.g., enum values).

## The `#[verifier::external_body]` Annotation

This annotation tells Verus to skip verification of a function's body, trusting only its signature and contracts. Use it for:

- **Logging functions** -- String formatting is not verifiable
- **Functions with complex runtime logic** -- Array indexing, match on computed indices, or other patterns Verus cannot reason about

```rust
#[verifier::external_body]
pub fn complex_helper(data: &[i32]) -> i32 {
    // Verus won't try to verify this body
    data.iter().sum()
}
```

Use this sparingly. Prefer keeping functions verifiable when possible.

## Enum Comparison in Verus: Use Pattern Matching

Inside `verus!` blocks, **do not use `==` to compare enum values** from the `data` crate. The `PartialEq` trait is derived on enums in the `data` crate, which is outside `verus!` blocks. Verus treats such externally-defined functions as opaque and will reject their use in verified code with an error like:

```
error: cannot use function `data::..::impl&%11::eq` which is ignored because it is
       either declared outside the verus! macro or it is marked as `external`.
```

Instead, use `match` to dispatch on enum variants. Verus can reason about pattern matching natively without needing external function specifications.

```rust
// WRONG: Verus cannot verify the externally-defined == operator
if msg.security_level == SNG_Data_Model::SecurityLevel::TopSecret {
    // ...
}

// CORRECT: Verus handles pattern matching natively
match msg.security_level {
    SNG_Data_Model::SecurityLevel::TopSecret => {
        // ...
    }
    _ => {
        // ...
    }
}
```

## Complete Example: Gate Component

This component drops TopSecret messages and passes others through:

```rust
use data::*;
use crate::bridge::gate_gate_api::*;
use vstd::prelude::*;

verus! {
    pub struct gate_gate {
    }

    impl gate_gate {
        pub fn new() -> Self {
            Self { }
        }

        pub fn initialize<API: gate_gate_Put_Api>(
            &mut self,
            api: &mut gate_gate_Application_Api<API>)
        {
            log_info("initialize entrypoint invoked");
            // EventDataPort -- no initialization needed
        }

        pub fn timeTriggered<API: gate_gate_Full_Api>(
            &mut self,
            api: &mut gate_gate_Application_Api<API>)
        {
            let input_contents = api.get_input();
            match input_contents {
                Some(msg) => {
                    match msg.security_level {
                        SNG_Data_Model::SecurityLevel::TopSecret => {
                            log_message_dropped(msg);
                        }
                        _ => {
                            api.put_output(msg);
                            log_message_passed(msg);
                        }
                    }
                }
                None => { }
            };
        }

        pub fn notify(&mut self, channel: microkit_channel) {
            match channel {
                _ => { log_warn_channel(channel) }
            }
        }
    }

    #[verifier::external_body]
    pub fn log_info(msg: &str) {
        log::info!("{0}", msg);
    }

    #[verifier::external_body]
    pub fn log_message_dropped(msg: SNG_Data_Model::Message) {
        log::info!("Gate: DROPPED message (security_level={0:?}, payload={1})",
            msg.security_level, msg.payload);
    }

    #[verifier::external_body]
    pub fn log_message_passed(msg: SNG_Data_Model::Message) {
        log::info!("Gate: PASSED message (security_level={0:?}, payload={1})",
            msg.security_level, msg.payload);
    }

    #[verifier::external_body]
    pub fn log_warn_channel(channel: u32) {
        log::warn!("Unexpected channel: {0}", channel);
    }
}
```

## Complete Example: Filter Component

This component clamps Secret message payloads to [0,100] and passes Public messages unchanged:

```rust
use data::*;
use crate::bridge::msg_filter_msg_filter_api::*;
use vstd::prelude::*;

verus! {
    pub struct msg_filter_msg_filter {
    }

    impl msg_filter_msg_filter {
        pub fn new() -> Self {
            Self { }
        }

        pub fn initialize<API: msg_filter_msg_filter_Put_Api>(
            &mut self,
            api: &mut msg_filter_msg_filter_Application_Api<API>)
        {
            log_info("initialize entrypoint invoked");
        }

        pub fn timeTriggered<API: msg_filter_msg_filter_Full_Api>(
            &mut self,
            api: &mut msg_filter_msg_filter_Application_Api<API>)
        {
            let input_contents = api.get_input();
            match input_contents {
                Some(msg) => {
                    match msg.security_level {
                      SNG_Data_Model::SecurityLevel::Public => {
                        api.put_output(msg);
                        log_message_passed(msg);
                      }
                      _ => {
                        // Secret messages: clamp payload to [0, 100]
                        let clamped_payload: i32;
                        if msg.payload > 100 {
                            clamped_payload = 100;
                        } else if msg.payload < 0 {
                            clamped_payload = 0;
                        } else {
                            clamped_payload = msg.payload;
                        }
                        let output_msg = SNG_Data_Model::Message {
                            security_level: msg.security_level,
                            payload: clamped_payload,
                        };
                        api.put_output(output_msg);
                        log_message_filtered(msg, output_msg);
                      }
                    }
                }
                None => { }
            };
        }

        pub fn notify(&mut self, channel: microkit_channel) {
            match channel {
                _ => { log_warn_channel(channel) }
            }
        }
    }

    #[verifier::external_body]
    pub fn log_info(msg: &str) {
        log::info!("{0}", msg);
    }

    #[verifier::external_body]
    pub fn log_message_passed(msg: SNG_Data_Model::Message) {
        log::info!("Filter: PASSED Public message unchanged (payload={0})",
            msg.payload);
    }

    #[verifier::external_body]
    pub fn log_message_filtered(input: SNG_Data_Model::Message, output: SNG_Data_Model::Message) {
        log::info!("Filter: Secret message filtered (payload: {0} -> {1})",
            input.payload, output.payload);
    }

    #[verifier::external_body]
    pub fn log_warn_channel(channel: u32) {
        log::warn!("Unexpected channel: {0}", channel);
    }
}
```

## Key Auto-Generated Files (Do Not Edit)

| File | Purpose |
|------|---------|
| `bridge/<component>_api.rs` | Port API traits and `Application_Api` struct with Verus contracts |
| `bridge/extern_c_api.rs` | C FFI bindings (production) or Mutex statics (test mode) |
| `bridge/<component>_GUMBOX.rs` | Executable GUMBO contracts as boolean functions |
| `lib.rs` | Crate root: static state, extern "C" entry points, module wiring |
