# Simple Network Guard (SNG)

This file contains a *sketch* of architectural and functional requirements for the SNG system to be implemented using HAMR with the seL4 Microkit target.  Despite the file name, the contents are not strictly limited to requirements, nor are the requirements rigorously organized; the file also includes general workflow suggestions and concepts to explore in the development of the system.

This project is a classroom illustration (this is a toy example) and therefore the communication of the network aspects is simulated, and the format of messages is dramatically simplified.

The purpose of the system is to illustrate basic concepts of network message processing that might be found in a network guard (aka "cross domain solution") including...
  - dropping messages whose fields do not satisfy some conditions
  - sanitizing the contents of messages fields

The SNG implementation artifacts include simple system test harness to send simulated messages into the system and to receive simulated messages coming out of the system.  By examining the system inputs and outputs, the test harness will be able to determine if the guard is performing correctly.

# Phase 1 - Modeling and Initial Implementation

## System Boundary and External Interfaces

### Inputs
  
  - The system inputs include
      - an ingress port that receives messages from the system context

### Outputs      

  - The system outputs include
      - an egress port that publishes any messages passing the guard into the system context

## Data Requirements

- SNG shall process messages with two fields:
    - security_level - security level of message
        Values: Public, Restricted, Critical
    - payload - message payload
        Values: 32-bit signed integers

## System Requirements

- Req_C: No critical messages received through the ingress port are emitted through the egress port (all critical messages are dropped)
- Req_R_1: All restricted messages received through the ingress port shall be emitted through the egress port
- Req_R_2: Each restricted message InR received through the ingress port shall have a modified version OutR flowing through the output port with the following relationship between InR and OutR:
   (a) if the payload of InR is greater than 100, the payload  of OutR is modified to have the value of 100,
   (b) if the payload of InR is less than 0, the payload of OutR is modified to have the value of 0, and
   (c) if the payload of InR is greater than or equal to 0 and less than or equal to 100, the payload of OutR is unchanged
- Req_P: All public messages received through the ingress port are emitted through the egress port with their contents unchanged

## Design Expectations

The SNG is implemented as a pipeline with two stages:
  - Gate - responsible for implementing message drop/pass policies, i.e., decides whether messages get passed to the next stage of the pipeline (and thus out the egress port) or are dropped
  - Filter - responsible for modifying the payload contents according to the stated requirements

Pipeline stages should be implemented to ensure independence of the stages (i.e., non-interference)

# Phase 2 - Adding GUMBO Documentation, More Tests, and Verus Verification

Note: The phasing DOES NOT imply a recommended ordering of workflow steps (we don't mean to imply that initial code should always be written before writing contracts).  The given order simply follows the ordering of training/lecture material in which GUMBO concepts are introduced after students learn to write basic models and code with HAMR.

## Overall goals and learning objectives

This phase addresses the following workflow activities...
 - apply HAMR SysMLv2 Type Checking, HAMR SysMLv2 Logika Check in the CodeIVE to check model-level specifications.
 - run HAMR code generation to update code-level Verus contracts and GUMBOX testing infrastructure
 - use Verus to verify that the gate and filter components satisfy their specifications
 - use HAMR testing, including GUMBOX property-based testing infrastructure to test that the gate and filter components satisfy their specifications.  Testing should include some manual GUMBOX tests to provide validation of the specifications (i.e., do the specifications match the stakeholders mental model fo the system).
 
## Refining High-Level Natural Language System Requirements to Component-Level Natural Language Requirements

For the decomposition of natural language requirements to HAMR component requirements, illustrate..
 - writing natural language requirements in a structured way to faciliate the eventual mapping to the GUMBO specifications
 - traceability from the component requirements back to the system requirements
 - construction of a "glossary" that provides a canonical application domain terminology with associated definitions

## GUMBO Component Specifications

For the GUMBO specifications, here are some key features that are to be illustrated:
 - use of GUMBO library functions to specify the policies associated with gate and filter components
 - use GUMBO "guarantee" clauses only for the gate component compute contract to illustrate how the use of the "implies" key word can give the same semantics as GUMBO case assume / guarantee pairs
 - use of the HasEvent and NoEvent GUMBO constructs to reason about the presence or absence of messages on ports
 - use of the short circuit GUMBO "and" operation and HasEvent to guard other expressions that access port messages (avoiding erroneous access of an empty port)
 - use of GUMBO library functions in integration constraints to summarize important invariants that should hold on ports.
   These same GUMBO library functions may also be used in GUMBO compute and initialize specifications.

Student tasks
- following documented HAMR process steps, develop a model solution illustrating the above concepts based on the existing Simple Network Guard (SNG) requirements, models, and code.
- if possible, the model solution should avoid changing the model-level component definitions (only add new GUMBO features).  There may be situations where you used to Rust idioms (e.g., comparing enum values) that aren't supported by Verus, and you will need to refactor
- in the process of building the model solution, note if adding the specifications, testing, and verification uncovered any errors in the application code of the gate and filter components
- there will be occasions where Verus will report possible overflow errors associated with non-GUMBO (e.g., in test_sender and test_receiver).  You will need to think about strategies for getting those to go away (via adding additional specs outside of GUMBO markers and by making some small code modifications).











