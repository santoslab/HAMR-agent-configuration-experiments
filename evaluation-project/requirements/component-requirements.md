# SNG Component Requirements

This document decomposes the system-level requirements from `requirements.md` into component-level requirements for the Gate and Filter pipeline stages. Requirements are written in structured natural language to facilitate mapping to GUMBO formal specifications.

## Glossary

| Term | Definition |
|------|-----------|
| **Message** | A data structure consisting of a `security_level` field and a `payload` field |
| **SecurityLevel** | An enumeration with values: `Public`, `Restricted`, `Critical` |
| **payload** | A signed 32-bit integer value carried by a Message |
| **allowed security level** | A security level that is either `Public` or `Restricted` (i.e., not `Critical`) |
| **clamped payload** | A payload value within the range [0, 100] inclusive |
| **clamped payload lower bound** | The value 0 — the minimum allowed clamped payload |
| **clamped payload upper bound** | The value 100 — the maximum allowed clamped payload |
| **HasEvent** | Indicates that a message is present on an event data port during the current dispatch |
| **NoEvent** | Indicates that no message is present on an event data port during the current dispatch |
| **equal message** | Two messages are equal when they have the same security level and the same payload |

## Gate Component Requirements

The Gate is the first stage of the guard pipeline. It implements message drop/pass policies: Critical messages are dropped, all other messages pass through unchanged.

### Compute Requirements

| Req ID | Traces To | Requirement |
|--------|-----------|-------------|
| **Gate_Req_C** | Req_C | When a message with `Critical` security level is present on the input port (HasEvent), no message shall be placed on the output port (NoEvent) |
| **Gate_Req_R1** | Req_R_1 | When a message with `Restricted` security level is present on the input port (HasEvent), an equal message shall be placed on the output port (HasEvent) |
| **Gate_Req_P** | Req_P | When a message with `Public` security level is present on the input port (HasEvent), an equal message shall be placed on the output port (HasEvent) |
| **Gate_Req_NoInput** | — | When no message is present on the input port (NoEvent), no message shall be placed on the output port (NoEvent) |

### Integration Requirements

| Req ID | Traces To | Requirement |
|--------|-----------|-------------|
| **Gate_Int_Output** | Req_C | Any message placed on the output port shall have an allowed security level |

## Filter Component Requirements

The Filter is the second stage of the guard pipeline. It implements payload sanitization: Restricted message payloads are clamped to the range [0, 100], Public messages pass through unchanged. The Filter relies on the Gate upstream to ensure that Critical messages never arrive.

### Compute Requirements

| Req ID | Traces To | Requirement |
|--------|-----------|-------------|
| **Filter_Req_P** | Req_P | When a message with `Public` security level is present on the input port (HasEvent), an equal message shall be placed on the output port (HasEvent) |
| **Filter_Req_R2** | Req_R_2 | When a message with `Restricted` security level is present on the input port (HasEvent), a message shall be placed on the output port (HasEvent) with the same security level, a clamped payload, and — if the input payload was already clamped — the same payload as the input |
| **Filter_Req_NoInput** | — | When no message is present on the input port (NoEvent), no message shall be placed on the output port (NoEvent) |

### Integration Requirements

| Req ID | Traces To | Requirement |
|--------|-----------|-------------|
| **Filter_Int_Input** | Gate_Int_Output | Any message present on the input port shall have an allowed security level |

## Traceability Summary

| System Requirement | Gate Component | Filter Component |
|--------------------|----------------|------------------|
| Req_C | Gate_Req_C, Gate_Int_Output | — (excluded by Filter_Int_Input) |
| Req_R_1 | Gate_Req_R1 | — |
| Req_R_2 | — | Filter_Req_R2 |
| Req_P | Gate_Req_P | Filter_Req_P |

**Pipeline invariant:** Gate_Int_Output guarantees that only messages with allowed security levels reach the Filter. Filter_Int_Input assumes this guarantee, establishing a compositional contract between the two pipeline stages.
