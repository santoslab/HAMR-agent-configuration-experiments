# Change Report CR-002: GUMBO Formal Specifications for Gate and Filter

| Field | Value |
|-------|-------|
| **Change ID** | CR-002 |
| **Date** | 2026-04-02 |
| **Summary** | Add comprehensive GUMBO formal specifications (compute contracts, integration constraints, library functions) for Gate and Filter components |
| **Scope** | Requirements, SysMLv2 models (GUMBOLib, SNG_Software), HAMR-generated code |
| **Verification** | Type checker: Well-formed; Logika integration check: Verified; Verus: Gate 8/0, Filter 8/0; Tests: 60 passed, 0 failed |

---

## 1. Requirements Changes

**File:** `requirements/component-requirements.md` (new file)

This change introduced a structured component requirements document that decomposes the existing system-level requirements (`requirements/requirements.md`) into component-level requirements for Gate and Filter. The system-level requirements were not modified.

### 1.1 New Artifacts

| Artifact | Description |
|----------|-------------|
| `requirements/component-requirements.md` | Structured component requirements with glossary, traceability to system requirements, and terminology aligned with GUMBO constructs |

### 1.2 Component Requirements Introduced

#### Gate Compute Requirements

| Req ID | Traces To | Requirement Summary |
|--------|-----------|---------------------|
| Gate_Req_C | Req_C | Critical input → no output (drop) |
| Gate_Req_R1 | Req_R_1 | Restricted input → equal message output (pass) |
| Gate_Req_P | Req_P | Public input → equal message output (pass) |
| Gate_Req_NoInput | — | No input → no output |

#### Gate Integration Requirements

| Req ID | Traces To | Requirement Summary |
|--------|-----------|---------------------|
| Gate_Int_Output | Req_C | Output always has allowed security level |

#### Filter Compute Requirements

| Req ID | Traces To | Requirement Summary |
|--------|-----------|---------------------|
| Filter_Req_P | Req_P | Public input → equal message output (pass) |
| Filter_Req_R2 | Req_R_2 | Restricted input → output with same security level, clamped payload; unchanged if already clamped |
| Filter_Req_NoInput | — | No input → no output |

#### Filter Integration Requirements

| Req ID | Traces To | Requirement Summary |
|--------|-----------|---------------------|
| Filter_Int_Input | Gate_Int_Output | Input always has allowed security level (pipeline invariant) |

### 1.3 Glossary

The component requirements document introduced a canonical glossary defining: Message, SecurityLevel, payload, allowed security level, clamped payload, clamped payload lower/upper bounds, HasEvent, NoEvent, and equal message. These terms directly map to GUMBO specification elements.

---

## 2. Model Changes

**Validated by:** HAMR type checker (`sireum_hamr_sysml_tipe`) — result: **Well-formed**
**Integration check:** HAMR Logika (`sireum_hamr_sysml_logika`) — result: **Integration constraints verified** (gate_to_filter connection)

### 2.1 GUMBOLib.sysml (new GUMBO content)

**File:** `sysmlv2/sng/GUMBOLib.sysml`

The GUMBOLib file was previously present but its GUMBO content was commented out. This change uncommented and corrected the GUMBO library block.

| Element | Description | Traces to Requirement |
|---------|-------------|-----------------------|
| `clampedPayloadLowerBound()` | Returns `0[i32]` — lower clamping bound | Glossary: clamped payload lower bound |
| `clampedPayloadUpperBound()` | Returns `100[i32]` — upper clamping bound | Glossary: clamped payload upper bound |
| `clampedPayload(m)` | `lowerBound <= m.payload <= upperBound` | Glossary: clamped payload; Filter_Req_R2 |
| `allowedSecurityLevel(m)` | `m.security_level == Restricted or Public` | Glossary: allowed security level; Gate_Int_Output, Filter_Int_Input |
| `equalSecurityLevel(m1, m2)` | `m1.security_level == m2.security_level` | Glossary: equal message (partial) |
| `equalPayload(m1, m2)` | `m1.payload == m2.payload` | Glossary: equal message (partial) |
| `equalMessage(m1, m2)` | `equalSecurityLevel and equalPayload` | Glossary: equal message |

#### Syntax corrections applied during type checking:
- Block delimiters: `{ }` → `/*{ }*/` (required GUMBO syntax)
- Enum value references: `SecurityLevel::Restricted` → `SecurityLevel.Restricted` (dot notation for enum values in GUMBO)

### 2.2 SNG_Software.sysml (GUMBO specifications added)

**File:** `sysmlv2/sng/SNG_Software.sysml`

#### Gate Thread — GUMBO Specifications Added

| GUMBO Element | Clause Type | Specification | Traces to |
|---------------|-------------|---------------|-----------|
| `No_Critical_Output` | integration guarantee | `GumboLib::GUMBO__Library::allowedSecurityLevel(output)` | Gate_Int_Output |
| `Req_C_Drop_Critical` | compute guarantee | `HasEvent(input) and (input.security_level == Critical) implies NoSend(output)` | Gate_Req_C |
| `Req_R1_Pass_Restricted` | compute guarantee | `HasEvent(input) and (input.security_level == Restricted) implies HasEvent(output) and equalMessage(input, output)` | Gate_Req_R1 |
| `Req_P_Pass_Public` | compute guarantee | `HasEvent(input) and (input.security_level == Public) implies HasEvent(output) and equalMessage(input, output)` | Gate_Req_P |
| `No_Input_No_Output` | compute guarantee | `(not HasEvent(input)) implies NoSend(output)` | Gate_Req_NoInput |

**Design note:** Gate uses guarantee-only clauses with `implies` to illustrate how implies can give the same semantics as case assume/guarantee pairs.

#### Filter Thread — GUMBO Specifications Added

| GUMBO Element | Clause Type | Specification | Traces to |
|---------------|-------------|---------------|-----------|
| `No_Critical_Input` | integration assume | `GumboLib::GUMBO__Library::allowedSecurityLevel(input)` | Filter_Int_Input |
| `Req_P_Public_Pass` | compute_cases case | assume: `HasEvent(input) and Public`; guarantee: `HasEvent(output) and equalMessage(input, output)` | Filter_Req_P |
| `Req_R2_Restricted_Clamp` | compute_cases case | assume: `HasEvent(input) and Restricted`; guarantee: `HasEvent(output) and equalSecurityLevel and clampedPayload(output) and (clampedPayload(input) implies equalPayload)` | Filter_Req_R2 |
| `No_Input` | compute_cases case | assume: `not HasEvent(input)`; guarantee: `NoSend(output)` | Filter_Req_NoInput |

**Design note:** Filter uses `compute_cases` with case assume/guarantee pairs to contrast with Gate's guarantee-only style.

#### Import and Syntax Changes

| Change | Before | After |
|--------|--------|-------|
| Added import | — | `private import GumboLib::*;` |
| GUMBO delimiters | `language "GUMBO" { ... }` | `language "GUMBO" /*{ ... }*/` |
| Library function references | `GumboLib::fn(...)` | `GumboLib::GUMBO__Library::fn(...)` |
| No-event on output | `NoEvent(output)` | `NoSend(output)` |
| No-event on input | `NoEvent(input)` | `not HasEvent(input)` |
| `compute_cases` placement | Standalone at GUMBO block level | Nested inside `compute` block |

### 2.3 Integration Constraint Verification

The Logika verifier confirmed that the Gate's output guarantee (`allowedSecurityLevel`) satisfies the Filter's input assumption (`allowedSecurityLevel`) across the `gate_to_filter` connection, establishing the pipeline invariant compositionally.

---

## 3. Code Changes

**Process:** HAMR code generation executed to update auto-generated files; developer-written files had contracts woven into MARKER regions automatically.
**Verus verification:** Gate: 8 verified, 0 errors; Filter: 8 verified, 0 errors
**Tests:** 60 tests passed across all 4 component crates

### 3.1 Auto-Generated Files (updated by HAMR code generation)

| File | Change Description | Traces to Model Element |
|------|--------------------|------------------------|
| `crates/GumboLib/src/lib.rs` | Generated Rust executable functions (`clampedPayload`, `allowedSecurityLevel`, `equalMessage`, etc.) and Verus spec functions (`_spec` variants) from GUMBO library definitions | GUMBOLib.sysml library functions |
| `crates/gate_gate/src/bridge/gate_gate_api.rs` | Verus `requires` on `put_output()` now enforces `allowedSecurityLevel_spec`; Verus `ensures` on compute entry point includes all 4 guarantee clauses | Gate GUMBO integration + compute |
| `crates/gate_gate/src/bridge/gate_gate_GUMBOX.rs` | New file: executable GUMBO contracts — `I_Guar_output`, `compute_spec_Req_C_Drop_Critical_guarantee`, `compute_spec_Req_R1_Pass_Restricted_guarantee`, `compute_spec_Req_P_Pass_Public_guarantee`, `compute_spec_No_Input_No_Output_guarantee`, `compute_CEP_Post` | Gate GUMBO compute guarantees |
| `crates/gate_gate/src/test/util/cb_apis.rs` | Contract-based test harness: `testComputeCB`, `testInitializeCB`, PropTest macros using GUMBOX contracts | Gate GUMBO contracts |
| `crates/msg_filter_msg_filter/src/bridge/msg_filter_msg_filter_api.rs` | Verus `ensures` on `get_input()` now provides `allowedSecurityLevel_spec` assumption; Verus `ensures` on compute includes 3 case contracts | Filter GUMBO integration + compute_cases |
| `crates/msg_filter_msg_filter/src/bridge/msg_filter_msg_filter_GUMBOX.rs` | New file: executable GUMBO contracts — `I_Assm_input`, `compute_case_Req_P_Public_Pass`, `compute_case_Req_R2_Restricted_Clamp`, `compute_case_No_Input`, `compute_CEP_Pre`, `compute_CEP_Post` | Filter GUMBO compute_cases |
| `crates/msg_filter_msg_filter/src/test/util/cb_apis.rs` | Contract-based test harness with precondition checking (rejects Critical inputs) | Filter GUMBO contracts |
| `crates/*/src/test/util/generators.rs` | PropTest generators updated (all 4 component crates) | Data model |
| `crates/*/src/test/util/test_apis.rs` | Test port APIs updated (all 4 component crates) | Component interfaces |
| `crates/*/src/lib.rs` | Crate root module wiring updated (all 4 component crates) | Component structure |

### 3.2 Developer-Written Files (contracts woven by HAMR)

These files are marked `// This file will not be overwritten if codegen is rerun`. HAMR preserved all developer application code and wove updated Verus contracts into the MARKER regions.

#### 3.2.1 Component Implementations

| File | Change Description | Traces to |
|------|--------------------|-----------|
| `crates/gate_gate/src/component/gate_gate_app.rs` | HAMR wove 4 `ensures` clauses into `timeTriggered` (lines 42–56): `Req_C_Drop_Critical`, `Req_R1_Pass_Restricted`, `Req_P_Pass_Public`, `No_Input_No_Output`. Developer application logic unchanged. | Gate_Req_C, Gate_Req_R1, Gate_Req_P, Gate_Req_NoInput |
| `crates/msg_filter_msg_filter/src/component/msg_filter_msg_filter_app.rs` | HAMR wove 3 case `ensures` clauses into `timeTriggered` (lines 42–56): `Req_P_Public_Pass`, `Req_R2_Restricted_Clamp`, `No_Input`. Developer application logic unchanged. | Filter_Req_P, Filter_Req_R2, Filter_Req_NoInput |

**Key finding:** No changes were needed to the developer-written application logic. The existing Gate and Filter implementations already satisfied the new GUMBO specifications, as confirmed by Verus verification (0 errors).

#### 3.2.2 Test Files

No changes were made to the existing test files (`tests.rs`). The existing tests (manual unit tests, GUMBOX contract-based tests, and PropTest property-based tests) all pass against the updated GUMBOX contracts.

---

## 4. Traceability Matrices

### 4.1 Requirements to Model

| Requirement | Model Element(s) | Model File |
|-------------|-------------------|------------|
| Gate_Req_C (Req_C) | GUMBO compute guarantee `Req_C_Drop_Critical` | `SNG_Software.sysml:54–56` |
| Gate_Req_R1 (Req_R_1) | GUMBO compute guarantee `Req_R1_Pass_Restricted` | `SNG_Software.sysml:58–60` |
| Gate_Req_P (Req_P) | GUMBO compute guarantee `Req_P_Pass_Public` | `SNG_Software.sysml:62–64` |
| Gate_Req_NoInput | GUMBO compute guarantee `No_Input_No_Output` | `SNG_Software.sysml:66–67` |
| Gate_Int_Output (Req_C) | GUMBO integration guarantee `No_Critical_Output` | `SNG_Software.sysml:50–51` |
| Filter_Req_P (Req_P) | GUMBO compute_cases case `Req_P_Public_Pass` | `SNG_Software.sysml:97–99` |
| Filter_Req_R2 (Req_R_2) | GUMBO compute_cases case `Req_R2_Restricted_Clamp` | `SNG_Software.sysml:101–106` |
| Filter_Req_NoInput | GUMBO compute_cases case `No_Input` | `SNG_Software.sysml:108–110` |
| Filter_Int_Input (Gate_Int_Output) | GUMBO integration assume `No_Critical_Input` | `SNG_Software.sysml:92–93` |
| Glossary terms | GUMBO library functions in GumboLib | `GUMBOLib.sysml:8–29` |

### 4.2 Model to Code (Auto-Generated)

| Model Element | Generated Code Artifact(s) |
|---------------|----------------------------|
| GumboLib library functions | `crates/GumboLib/src/lib.rs` — Rust executable + Verus spec functions |
| Gate `No_Critical_Output` | `gate_gate_api.rs` — Verus `requires` on `put_output()`; `gate_gate_GUMBOX.rs` — `I_Guar_output()` |
| Gate `Req_C_Drop_Critical` | `gate_gate_app.rs` — Verus `ensures` clause; `gate_gate_GUMBOX.rs` — `compute_spec_Req_C_Drop_Critical_guarantee()` |
| Gate `Req_R1_Pass_Restricted` | `gate_gate_app.rs` — Verus `ensures` clause; `gate_gate_GUMBOX.rs` — `compute_spec_Req_R1_Pass_Restricted_guarantee()` |
| Gate `Req_P_Pass_Public` | `gate_gate_app.rs` — Verus `ensures` clause; `gate_gate_GUMBOX.rs` — `compute_spec_Req_P_Pass_Public_guarantee()` |
| Gate `No_Input_No_Output` | `gate_gate_app.rs` — Verus `ensures` clause; `gate_gate_GUMBOX.rs` — `compute_spec_No_Input_No_Output_guarantee()` |
| Filter `No_Critical_Input` | `msg_filter_msg_filter_api.rs` — Verus `ensures` on `get_input()`; `msg_filter_msg_filter_GUMBOX.rs` — `I_Assm_input()` |
| Filter `Req_P_Public_Pass` | `msg_filter_msg_filter_app.rs` — Verus `ensures` clause; `msg_filter_msg_filter_GUMBOX.rs` — `compute_case_Req_P_Public_Pass()` |
| Filter `Req_R2_Restricted_Clamp` | `msg_filter_msg_filter_app.rs` — Verus `ensures` clause; `msg_filter_msg_filter_GUMBOX.rs` — `compute_case_Req_R2_Restricted_Clamp()` |
| Filter `No_Input` | `msg_filter_msg_filter_app.rs` — Verus `ensures` clause; `msg_filter_msg_filter_GUMBOX.rs` — `compute_case_No_Input()` |

### 4.3 Requirements to Code (Developer-Written)

| Requirement | Code Implementation | Code File |
|-------------|--------------------|----|
| Gate_Req_C | `SecurityLevel::Critical => { log_message_dropped(msg); }` | `gate_gate_app.rs:67–69` |
| Gate_Req_R1 | `_ => { api.put_output(msg); }` (Restricted falls through Critical match) | `gate_gate_app.rs:71–74` |
| Gate_Req_P | `_ => { api.put_output(msg); }` (Public falls through Critical match) | `gate_gate_app.rs:71–74` |
| Gate_Req_NoInput | `None => { }` (no action when no input) | `gate_gate_app.rs:78–80` |
| Filter_Req_P | `SecurityLevel::Public => { api.put_output(msg); }` | `msg_filter_msg_filter_app.rs:68–71` |
| Filter_Req_R2 | Payload clamping logic: `if msg.payload > 100 { 100 } else if msg.payload < 0 { 0 } else { msg.payload }` | `msg_filter_msg_filter_app.rs:74–91` |
| Filter_Req_NoInput | `None => { }` (no action when no input) | `msg_filter_msg_filter_app.rs:95–97` |

### 4.4 Requirements to Tests

| Requirement | Test Function(s) | Test File |
|-------------|-------------------|----|
| Gate_Req_C | `test_Req_C_drop_critical`, `test_Req_C_drop_critical_negative_payload`, `test_Req_C_drop_critical_zero_payload`, `test_Req_C_drop_critical_max_payload` | `gate_gate/.../tests.rs` |
| Gate_Req_C | `test_GUMBOX_Req_C_critical_dropped` (contract-based) | `gate_gate/.../tests.rs` |
| Gate_Req_R1 | `test_Req_R1_pass_restricted`, `test_Req_R1_pass_restricted_negative_payload`, `test_Req_R1_pass_restricted_large_payload` | `gate_gate/.../tests.rs` |
| Gate_Req_R1 | `test_GUMBOX_Req_R1_restricted_passed` (contract-based) | `gate_gate/.../tests.rs` |
| Gate_Req_P | `test_Req_P_pass_public`, `test_Req_P_pass_public_zero_payload`, `test_Req_P_pass_public_min_payload` | `gate_gate/.../tests.rs` |
| Gate_Req_P | `test_GUMBOX_Req_P_public_passed` (contract-based) | `gate_gate/.../tests.rs` |
| Gate_Req_NoInput | `test_no_input` | `gate_gate/.../tests.rs` |
| Gate_Req_NoInput | `test_GUMBOX_no_input` (contract-based) | `gate_gate/.../tests.rs` |
| Gate_Int_Output | `test_integration_output_never_critical` | `gate_gate/.../tests.rs` |
| Filter_Int_Input | `test_GUMBOX_critical_rejected_precondition` (precondition rejection) | `msg_filter/.../tests.rs` |
| Filter_Req_P | `test_Req_P_public_unchanged`, `test_Req_P_public_zero_payload`, `test_Req_P_public_negative_payload`, `test_Req_P_public_large_payload`, `test_Req_P_public_min_payload`, `test_Req_P_public_max_payload` | `msg_filter/.../tests.rs` |
| Filter_Req_P | `test_GUMBOX_Req_P_public` (contract-based) | `msg_filter/.../tests.rs` |
| Filter_Req_R2 | `test_Req_R2a_*` (clamp above), `test_Req_R2b_*` (clamp below), `test_Req_R2c_*` (in range), `test_clamping_boundaries_comprehensive` | `msg_filter/.../tests.rs` |
| Filter_Req_R2 | `test_GUMBOX_Req_R2a_clamp_above`, `test_GUMBOX_Req_R2b_clamp_below`, `test_GUMBOX_Req_R2c_in_range` (contract-based) | `msg_filter/.../tests.rs` |
| Filter_Req_NoInput | `test_no_input` | `msg_filter/.../tests.rs` |
| Filter_Req_NoInput | `test_GUMBOX_no_input` (contract-based) | `msg_filter/.../tests.rs` |
| All Gate | `prop_testComputeCB_macro`, `prop_testComputeCB_Critical_biased`, `prop_testComputeCB_always_some`, `prop_testInitializeCB_macro` (property-based) | `gate_gate/.../tests.rs` |
| All Filter | `prop_testComputeCB_macro`, `prop_testComputeCB_no_rejections`, `prop_testComputeCB_restricted_boundary`, `prop_testComputeCB_always_some_valid`, `prop_testInitializeCB_macro` (property-based) | `msg_filter/.../tests.rs` |

### 4.5 Requirements to Formal Verification

| Requirement | Verification Method | Result |
|-------------|---------------------|--------|
| Gate_Int_Output + Filter_Int_Input | Logika integration constraint check (gate_to_filter connection) | **Verified** |
| Gate_Req_C | Verus postcondition on `timeTriggered` (`Req_C_Drop_Critical` ensures clause) | **Verified** (8/0) |
| Gate_Req_R1 | Verus postcondition on `timeTriggered` (`Req_R1_Pass_Restricted` ensures clause) | **Verified** (8/0) |
| Gate_Req_P | Verus postcondition on `timeTriggered` (`Req_P_Pass_Public` ensures clause) | **Verified** (8/0) |
| Gate_Req_NoInput | Verus postcondition on `timeTriggered` (`No_Input_No_Output` ensures clause) | **Verified** (8/0) |
| Filter_Req_P | Verus postcondition on `timeTriggered` (`Req_P_Public_Pass` case ensures) | **Verified** (8/0) |
| Filter_Req_R2 | Verus postcondition on `timeTriggered` (`Req_R2_Restricted_Clamp` case ensures) | **Verified** (8/0) |
| Filter_Req_NoInput | Verus postcondition on `timeTriggered` (`No_Input` case ensures) | **Verified** (8/0) |

---

## 5. Change Process Summary

| Step | Action | Tool/Method | Result |
|------|--------|-------------|--------|
| 1 | Create component requirements document | Manual authoring | `requirements/component-requirements.md` |
| 2 | Uncomment and correct GUMBOLib GUMBO library block | Manual edit | `sysmlv2/sng/GUMBOLib.sysml` |
| 3 | Add Gate GUMBO specifications (integration + compute guarantees) | Manual edit | `sysmlv2/sng/SNG_Software.sysml` |
| 4 | Add Filter GUMBO specifications (integration assume + compute_cases) | Manual edit | `sysmlv2/sng/SNG_Software.sysml` |
| 5 | Type check models | `sireum_hamr_sysml_tipe` | **Well-formed** (after syntax corrections) |
| 6 | Verify integration constraints | `sireum_hamr_sysml_logika` | **Integration constraints verified** |
| 7 | Generate code | `sireum_hamr_sysml_codegen` (Microkit) | Contracts woven into app files; GUMBOX and test infrastructure generated |
| 8 | Run Verus verification | `make verus` on gate_gate, msg_filter_msg_filter | Gate: 8 verified, 0 errors; Filter: 8 verified, 0 errors |
| 9 | Run tests | `make test` on all 4 component crates | 60 passed, 0 failed |

### Key GUMBO Syntax Issues Discovered and Corrected

These syntax issues were discovered during Step 5 (type checking) and represent lessons learned for GUMBO authoring:

| Issue | Incorrect | Correct |
|-------|-----------|---------|
| GUMBO block delimiters | `language "GUMBO" { ... }` | `language "GUMBO" /*{ ... }*/` |
| Library function qualified name | `GumboLib::functionName(...)` | `GumboLib::GUMBO__Library::functionName(...)` |
| Enum values in GUMBO | `SecurityLevel::Restricted` | `SecurityLevel.Restricted` |
| No-event on output port (guarantee) | `NoEvent(output)` | `NoSend(output)` |
| No-event on input port (assume/check) | `NoEvent(input)` | `not HasEvent(input)` |
| `compute_cases` placement | Standalone at GUMBO block level | Must be nested inside `compute` block |

### Application Code Errors Found

**None.** The existing Gate and Filter implementations were found to correctly satisfy all new GUMBO specifications without any code changes. Verus verified all postconditions, and all 60 tests (including GUMBOX contract-based tests) passed.
