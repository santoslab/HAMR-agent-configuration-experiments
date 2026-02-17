# Simple Network Guard (SNG)

This file contains a sketch of architectural and functional requirements for the SNG system to be implemented using HAMR with the seL4 Microkit target.  

This project is a classroom illustration (this is a toy example) and therefore the communication of the network aspects is simulated, and the format of messages is dramatically simplified.

The purpose of the system is to illustrate basic concepts of network message processing that might be found in a network guard (aka "cross domain solution") including...
  - dropping messages whose fields do not satisfy some conditions
  - sanitizing the contents of messages fields

The SNG implementation artifacts include simple system test harness to send simulated messages into the system and to receive simulated messages coming out of the system.  By examining the system inputs and outputs, the test harness will be able to determine if the guard is performing correctly.

# System Boundary and External Interfaces

## Inputs
  
  - The system inputs include
      - an ingress port that receives messages from the system context

## Outputs      

  - The system outputs include
      - an egress port that publishes any messages passing the guard into the system context

# Data Requirements

- SNG shall process messages with two fields:
    - security_level - security level of message
        Values: Public, Restricted, Critical
    - payload - message payload
        Values: 32-bit signed integers

# System Requirements

- Req_C: No critical messages received through the ingress port are emitted through the egress port (all critical messages are dropped)
- Req_R_1: All restricted messages received through the ingress port shall be emitted through the egress port
- Req_R_2: Each restricted message InR received through the ingress port shall have a modified version OutR flowing through the output port with the following relationship between InR and OutR:
   (a) if the payload of InR is greater than 100, the payload  of OutR is modified to have the value of 100,
   (b) if the payload of InR is less than 0, the payload of OutR is modified to have the value of 0, and
   (c) if the payload of InR is greater than or equal to 0 and less than or equal to 100, the payload of OutR is unchanged
- Req_P: All public messages received through the ingress port are emitted through the egress port with their contents unchanged

# Design Expectations

The SNG is implemented as a pipeline with two stages:
  - Gate - responsible for implementing message drop/pass policies, i.e., decides whether messages get passed to the next stage of the pipeline (and thus out the egress port) or are dropped
  - Filter - responsible for modifying the payload contents according to the stated requirements

Pipeline stages should be implemented to ensure independence of the stages (i.e., non-interference)









