# HAMR Exercise: Building a HAMR System from Scratch (Part 2 - Rust code) 

**Purpose**:  The purpose of this exercise is to get you familiar with generating code for a HAMR design system, completing the implementation of application components, and writing basic tests.  This assignment addresses development in SysMLv2 and Rust.

Starting from the model produced in Part 1 of this exercise, you will go through the steps of 
 - configurating the HAMR code generator in CodeIVE
 - running the HAMR code generator in CodeIVE
 - opening the Rust implementation of the HAMR generated thread components
 - navigating the different files at the code level
 - coding component entry points 
 - compiling your component code
 - adding some simple tests for your component code

## Prerequisites and Resources

Before working through this exercise, you should have gone through Part 1 of this exercise to build the described model, and you should have run `HAMR Type Checking` on the model to check its well-formedness.

You have set up the folder structure for this exercise with two top-level folders as follows:
 - sysmlv2
 - hamr

## Activity 1 - Setting Up a HAMR Code Generation Configuration

* **Task**: Working within the CodeIVE, in your "main" SysMLv2 file that includes the definition of the system component, use the CodeIVE command "HAMR SysML CodeGen Configurator" from the command palette to run code generation configuration tool.

Define a configuration for code generation for the Microkit platform, and set the `Output Directory` to be the `hamr` folder that you created in Part 1 of this exercise.  You can leave the "CAmkES / Microkit options" empty (we will just use the default values).

After you have completed the steps above, press the `Insert` button to insert the configured options into your SysMLv2 file.  The following configuration should appear at the top of the file.

```
//@ HAMR: --platform Microkit --output-dir ../../hamr
```

Based on the configuration above, the HAMR-generated project will be in `hamr` folder (as configured by the `output-dir` option above, and then in the `microkit` folder (the default name chosen for code generation when targetting the seL4 microkit platform).

## Activity 2 - Running HAMR Code Generation

* **Task**: Use the CodeIVE command "HAMR SysML CodeGen" from the command palette to run code generation, and select `Microkit` for the target platform. 

As the code generation runs, it will produce status information along with information about generated files in the CodeIVE Terminal pane.  Recall that successful code generation is indicated by the message...
```
info: Code generation successful!
```
..appearing at the end of code generation.

## Activity 2 - Commit the Results of Code Generation to Git and Observe the Results 

* **Task:** Commit and push to git the changes to your project that have been made from running HAMR code generation.

## Activity 3 - Adding the functionality for the Gate component

* **Task:** Using the `File / Open Folder` option in the CodeIVE (you may want to create a new window first), open the `gate_gate` crate for Gate component.  Navigate to the `src/component/gate_gate_app.rs` file and open it.  At this point, you should understand the purpose of all the auto-generated code for the file.  If you don't understand something, make note of it to investigate it in the HAMR documentation. 

Add documentation to the top of the file summarizing the purpose of this component (I suggest following the style of documentation found the previous HAMR examples -- particularly, the Simple Isolette).  You may want to indicate the specific requirements that this component is expected to implemented.  In the systems/software engineering community this is often referred to as *allocating* system requirements to subsystems or components.  Often in complex systems, a system requirement itself may need to be broken down (refined) to into something more specific for the component, but in this case the system requirement carries over fairly well to the component.

* **Task: Implementing the Gate Initialize entry point:** Scroll down in the app source code until you find the `initialize` method.   Recall that our main objectives for the `initialize` entry point is to implement initialization of any component state and also to initialize output data ports.  In this case, we don't have anything to initialize, so just make a comment to that effect in the initialize entry point.

```rust
// No output data ports to initialize (EventDataPort does not require initialization)
```

* **Task: Implementing the Gate Compute entry point (timeTriggered method):**
Scroll down in the app source code until you find the `timeTriggered` method.  Your previous documentation that indicated the specific system requirements allocated to this component should give you a hint at what you need to implement here (i.e., the drop/pass policies for messages based on their security level).

Here is a short summary of things you need to consider.
* Get the contents of the input port.  The result of the `get_` will be a value of `option` type.  See the Producer / Consumer example or the HAMR documentation to see the typical code style for getting a value from an `event data` port in a periodic thread.
* If there is no message on the port, you don't need to do anything else.
* If there is a message, extract the message from the `Some` alternative of the `option` type, examine/branch on the security level tag of the message, and then take appropriate action based on security level, e.g., `SNG_Data_Model::SecurityLevel::Critical`
* If a message is to be dropped, you can simply give an appropriage logging message (see below for hints) and do nothing else.
* If a message is to be pass through, you can give an appropriate logging message, and then call the `put` method for the output port to pass the message on through.

* **Task: Implementing logging methods:**
We'd like to have some informative logging methods.  In the exercise for the Simple Isolette example, we discussed that working with both Verus and seL4 requires us to make some extra effort beyond calling the usual Rust string formatting and output methods.  Using the logging methods that we added in the Simple Isolette as an example, add the following logging methods with the other auto-generated logging methods that can be called from the `timeTriggered` method above.

```rust
  #[verifier::external_body]
  pub fn log_message_dropped(msg: SNG_Data_Model::Message)
  {
    log::info!("Gate: DROPPED message (security_level={0:?}, payload={1})",
      msg.security_level, msg.payload);
  }

  #[verifier::external_body]
  pub fn log_message_passed(msg: SNG_Data_Model::Message)
  {
    log::info!("Gate: PASSED message (security_level={0:?}, payload={1})",
      msg.security_level, msg.payload);
  }
```

## Activity 4 - Writing Manual Tests for the Gate component

In this activity, you will write manual unit tests for the Gate component in `src/test/tests.rs`.  These tests directly verify that the Gate's input/output behavior matches the requirements you implemented in Activity 3.

* **Task: Understanding the testing pattern:**  Open the `src/test/tests.rs` file in the `gate_gate` crate.  You will see some auto-generated scaffolding code.  The pattern for testing a HAMR periodic component follows the same approach you saw in the Simple Isolette exercise:

1. Initialize the component (call the `initialize` entry point)
2. Set the input port value (using the test API `put_` method)
3. Run the compute entry point (`timeTriggered`)
4. Retrieve the output port value (using the test API `get_` method)
5. Assert the expected result

To reduce boilerplate, define a helper function `run_gate` that encapsulates steps 1–4:

```rust
fn run_gate(input: Option<SNG_Data_Model::Message>) -> Option<SNG_Data_Model::Message>
{
  crate::gate_gate_initialize();
  test_apis::put_input(input);
  crate::gate_gate_timeTriggered();
  test_apis::get_output()
}
```

This helper lets each test focus on constructing an input and asserting the output.  Note the use of `Option` — since the input port is an `EventDataPort`, there may or may not be a message present.

* **Task: Writing tests for each requirement:**  Write tests that cover each of the Gate's allocated requirements.  For each requirement, you should have at least one or two tests with different payloads.  Here is a summary of the coverage areas and some suggested test vectors to get you started:

**Req_C — Critical messages are dropped (output is `None`):**

| Security Level | Payload | Expected Output |
|---|---|---|
| `Critical` | `42` | `None` |
| `Critical` | `-100` | `None` |
| `Critical` | `0` | `None` |

For example, a test for Req_C would look like:
```rust
#[test]
#[serial]
fn test_Req_C_drop_critical() {
  let msg = SNG_Data_Model::Message {
    security_level: SNG_Data_Model::SecurityLevel::Critical,
    payload: 42,
  };
  let output = run_gate(Some(msg));
  assert!(output.is_none(), "Critical message should be dropped");
}
```

**Req_R_1 — Restricted messages pass through unchanged:**

| Security Level | Payload | Expected Output |
|---|---|---|
| `Restricted` | `50` | `Some(msg)` (unchanged) |
| `Restricted` | `-10` | `Some(msg)` (unchanged) |
| `Restricted` | `150` | `Some(msg)` (unchanged) |

Note that the Gate should *not* modify the payload of Restricted messages — payload clamping is handled by the downstream Message Filter component.

**Req_P — Public messages pass through unchanged:**

| Security Level | Payload | Expected Output |
|---|---|---|
| `Public` | `42` | `Some(msg)` (unchanged) |
| `Public` | `0` | `Some(msg)` (unchanged) |

**No input — when no message is present:**

When the input is `None`, the output should also be `None`.

* **Task: Adding boundary and edge-case tests:**  Add additional tests using boundary and extreme payload values such as `0`, `-1`, `i32::MIN`, `i32::MAX`.  The Gate's behavior should be purely based on the security level — the payload value should have no effect on whether a message is dropped or passed.  These tests help confirm that assumption.

* **Task: Testing the integration property:**  As a final test, write a test that verifies the following integration property: *the output of the Gate should never contain a message with `Critical` security level*.  You can do this by iterating over all three security levels, running each through the Gate, and asserting that any `Some` output does not have `SecurityLevel::Critical`.

## Activity 5 - Adding the functionality for the Message Filter component

* **Task:** Using the `File / Open Folder` option in the CodeIVE (you may want to create a new window first), open the `msg_filter_msg_filter` crate for Message Filter component.  Navigate to the `src/component/msg_filter_msg_filter_app.rs` file and open it.  

Add documentation to the top of the file summarizing the purpose of this component (I suggest following the style of documentation found the previous HAMR examples -- particularly, the Simple Isolette).  You may want to indicate the specific requirements that this component is expected to implemented.  

* **Task: Implementing the Message Filter Initialize entry point:** Scroll down in the app source code until you find the `initialize` method.   Similar to the Gate component, we don't have anything to initialize, so just make a comment to that effect in the initialize entry point.

```rust
// No output data ports to initialize (EventDataPort does not require initialization)
```

* **Task: Implementing the Gate Compute entry point (timeTriggered method):**
Scroll down in the app source code until you find the `timeTriggered` method.  Your previous documentation that indicated the specific system requirements allocated to this component should give you a hint at what you need to implement here (i.e., the drop/pass policies for messages based on their security level).

Here is a short summary of things you need to consider.
* Get the contents of the input port.  The result of the `get_` will be a value of `option` type.  
* If there is no message on the port, you don't need to do anything else.
* If there is a message, extract the message from the `Some` alternative of the `option` type, examine/branch on the security level tag of the message, and then take appropriate action based on security level, e.g., `Critical`, `Restricted`, `Public`.  Most of your will involve processing `Restricted` messages to implement the appropriate action.
* In the case where a new message payload constructed to reflect filtering (i.e., clamped to a particular range), you will need to use Rust struct constructor for `Message` as below to construct a value to send on the output port.
```
let output_msg = SNG_Data_Model::Message {
          security_level: msg.security_level,
          payload: clamped_payload,
        };
```

* **Task: Implementing logging methods:**
Similar to what you did for the Gate, add logging methods `log_message_passed` and `log_message_filtered` to accompany other auto-generated logging methods that can be called from the `timeTriggered` method above.
  - `log_message_passaged` should indicate that a public message was passed through with its payload unchanged
  - `log_message_filter` should indicate that a restricted message was passed through (show the original payload and the possibly filtered payload).


## Activity 6 - Writing Manual Tests for the Message Filter component

In this activity, you will write manual unit tests for the Message Filter component in `src/test/tests.rs` within the `msg_filter_msg_filter` crate.  The testing pattern is the same as Activity 4, adapted for the Filter's crate names and API functions.

* **Task: Setting up the test helper:**  Define a `run_filter` helper function similar to the `run_gate` helper you wrote for the Gate tests:

```rust
fn run_filter(input: Option<SNG_Data_Model::Message>) -> Option<SNG_Data_Model::Message>
{
  crate::msg_filter_msg_filter_initialize();
  test_apis::put_input(input);
  crate::msg_filter_msg_filter_timeTriggered();
  test_apis::get_output()
}
```

* **Task: Writing tests for each requirement:**  Write tests covering each of the Filter's allocated requirements.  The Filter has more nuanced behavior than the Gate because of payload clamping, so you will need more test cases — particularly around the clamping boundaries.

**Req_P — Public messages pass through unchanged (even with out-of-range payloads):**

| Security Level | Payload | Expected Output Payload |
|---|---|---|
| `Public` | `42` | `42` (unchanged) |
| `Public` | `-500` | `-500` (unchanged) |
| `Public` | `99999` | `99999` (unchanged) |

An important subtlety: Public messages are *not* subject to payload clamping — only Restricted messages are.  Make sure you test this with out-of-range payloads to confirm that the Filter does not accidentally clamp Public messages.

**Req_R_2a — Restricted payload > 100 is clamped to 100:**

| Security Level | Input Payload | Expected Output Payload |
|---|---|---|
| `Restricted` | `150` | `100` |
| `Restricted` | `101` | `100` |

For tests where the payload is modified, you need to construct the expected output message explicitly:
```rust
let expected = SNG_Data_Model::Message {
  security_level: SNG_Data_Model::SecurityLevel::Restricted,
  payload: 100,
};
assert!(output == Some(expected), "Restricted payload 150 should be clamped to 100");
```

**Req_R_2b — Restricted payload < 0 is clamped to 0:**

| Security Level | Input Payload | Expected Output Payload |
|---|---|---|
| `Restricted` | `-10` | `0` |
| `Restricted` | `-1` | `0` |

**Req_R_2c — Restricted payload in [0, 100] passes unchanged:**

| Security Level | Input Payload | Expected Output Payload |
|---|---|---|
| `Restricted` | `50` | `50` |
| `Restricted` | `0` | `0` |
| `Restricted` | `100` | `100` |

**No input — no message produces no output:**

When the input is `None`, the output should also be `None`.

* **Task: Emphasizing boundary testing:**  The clamping logic has two critical boundary points: `0` (lower bound) and `100` (upper bound).  Write tests that specifically target values around these boundaries: `-1`, `0`, `1`, `99`, `100`, `101`.  These are the values most likely to reveal off-by-one errors in your clamping implementation.  Consider writing a comprehensive boundary test that uses a table of `(input_payload, expected_output_payload)` pairs for Restricted messages and iterates through all of them.

* **Task: Verifying security level preservation:**  Write a test that confirms the security level in the output always matches the security level in the input.  For example, if you send a `Restricted` message with payload `999`, the output should still have `SecurityLevel::Restricted` (even though the payload was clamped).  Similarly, a `Public` message should come out with `SecurityLevel::Public`.

## Activity 7 - Adding the functionality for the Test Sender component

The Test Sender is a helper component whose purpose is to generate a predefined sequence of test messages that exercise all of the system's requirements during integration testing.  By cycling through a carefully chosen set of messages, you can observe (via log output from the Gate, Filter, and Test Receiver) whether the entire system behaves correctly end-to-end.

* **Task:** Using the `File / Open Folder` option in the CodeIVE, open the `test_sender_test_sender` crate.  Navigate to the `src/component/test_sender_test_sender_app.rs` file and open it.

* **Task: Adding a state variable:**  The Test Sender needs to track which test message to send next.  Add an `i32` field named `test_case_index` to the component's state struct.  This counter will cycle through a predefined set of test messages.  Also define a constant `NUM_TEST_CASES` (as a `const` just inside the `verus!` block) that holds the total number of test messages you plan to send.  Initialize `test_case_index` to `0` in both the `new()` constructor and the `initialize` entry point.

* **Task: Implementing the initialize entry point:**  In the `initialize` method, reset `test_case_index` to `0`.  There are no output data ports to initialize (the output is an EventDataPort which does not require initialization).

* **Task: Implementing the timeTriggered entry point:**  Each time the compute entry point fires, the Test Sender should:
  1. Build a test message based on the current value of `test_case_index`
  2. Send the message on the output port using `api.put_output(msg)`
  3. Log the message that was sent (including the case index, security level, and payload)
  4. Advance the counter and wrap it back to `0` when it reaches `NUM_TEST_CASES`

* **Task: Designing the test message table:**  Choose a set of 5–7 test messages that collectively exercise all the system requirements.  Your table should include messages with each security level and a mix of normal, boundary, and out-of-range payloads.  Here is a suggested set:

| Case Index | Security Level | Payload | Expected System Behavior |
|---|---|---|---|
| 0 | `Public` | `42` | Gate passes, Filter passes unchanged |
| 1 | `Restricted` | `50` | Gate passes, Filter passes unchanged (in range) |
| 2 | `Critical` | `99` | Gate DROPS |
| 3 | `Restricted` | `150` | Gate passes, Filter clamps to 100 |
| 4 | `Public` | `0` | Gate passes, Filter passes unchanged |
| 5 | `Restricted` | `-10` | Gate passes, Filter clamps to 0 |
| 6 | `Critical` | `200` | Gate DROPS |

You may adjust or extend this table as you see fit.

* **Task: Implementing the `build_test_message` helper function:**  Write a function `build_test_message(index: i32) -> SNG_Data_Model::Message` that returns the appropriate test message for the given index.  **Important:** use an if-else chain to select the message (not a `match` expression or an array lookup) because Verus has better compatibility with if-else chains.  The final `else` branch should handle any index outside the expected range (e.g., return the last test case).  Mark this function with `#[verifier::external_body]` since it uses patterns that Verus does not need to verify.

* **Task: Implementing the `log_message_sent` logging method:**  Add a logging method that displays the test case index, security level, and payload of the sent message.  For example:

```rust
#[verifier::external_body]
pub fn log_message_sent(index: i32, msg: SNG_Data_Model::Message)
{
  log::info!("TestSender: [case {0}] sent message (security_level={1:?}, payload={2})",
    index, msg.security_level, msg.payload);
}
```

You do not need to write any tests for this component

## Activity 8 - Adding the functionality for the Test Receiver component

The Test Receiver is a companion helper component to the Test Sender.  Its purpose is to receive messages that have passed through the Gate and Filter pipeline and log them so you can observe the end-to-end behavior of the system during integration testing.  By comparing the Test Sender's log output with the Test Receiver's log output, you can verify that the system correctly drops, passes, and clamps messages according to the requirements.

* **Task:** Using the `File / Open Folder` option in the CodeIVE, open the `test_receiver_test_receiver` crate.  Navigate to the `src/component/test_receiver_test_receiver_app.rs` file and open it.

* **Task: Adding a state variable:**  Add an `i32` field named `num_received` to the component's state struct.  This counter tracks how many messages have been received, which is useful for correlating log output with the Test Sender's numbered test cases.  Initialize `num_received` to `0` in both the `new()` constructor and the `initialize` entry point.

* **Task: Implementing the initialize entry point:**  In the `initialize` method, reset `num_received` to `0`.

* **Task: Implementing the timeTriggered entry point:**  Each time the compute entry point fires, the Test Receiver should:
  1. Get the contents of the input port using `api.get_input()` — this returns an `Option<SNG_Data_Model::Message>`
  2. If a message is present (`Some(msg)`):
     - Increment `num_received`
     - Log the received message (including the count, security level, and payload)
  3. If no message is present (`None`): do nothing

Use a `match` expression on the result of `get_input()` to handle the `Some` and `None` cases, following the same pattern used in the Gate and Filter components.

* **Task: Implementing the `log_message_received` logging method:**  Add a logging method that displays the running count of received messages along with the security level and payload.  For example:

```rust
#[verifier::external_body]
pub fn log_message_received(count: i32, msg: SNG_Data_Model::Message)
{
  log::info!("TestReceiver: [msg #{0}] received (security_level={1:?}, payload={2})",
    count, msg.security_level, msg.payload);
}
```

Mark this function with `#[verifier::external_body]` as discussed in the Simple Isolette exercise (logging functions use I/O that Verus cannot verify).

You do not need to write any tests for this component




