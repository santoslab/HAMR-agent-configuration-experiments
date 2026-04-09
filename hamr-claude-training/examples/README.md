# HAMR Examples for Claude Training

The example HAMR projects in this folder are provided as illustrations to Claude.

# Simple Isolette

@HAMR-SysMLv2-Rust-seL4-P-DP-Example illustrates HAMR used on SysMLv2 models and generating code for Rust component implementations deployed on seL4 Microkit.  The example illustrates periodic components with data ports.

# Producer / Consumer

@HAMR-SysMLv2-Rust-seL4-P-EDP-Prod-Cons-Example illustrates HAMR used on SysMLv2 models and generating code for Rust component implementations deployed on seL4 Microkit.  The example illustrates periodic components with event data ports.

# Simple Network Guard (SNG)

@HAMR-SysMLv2-Rust-seL4-P-EDP-SNG-Example illustrates HAMR used on SysMLv2 models and generating code for Rust component implementations deployed on seL4 Microkit.  The example illustrates a multi-component pipeline (test_sender -> gate -> msg_filter -> test_receiver) with periodic components using event data ports.  It demonstrates message filtering and dropping based on security levels, GUMBO integration constraints and compute contracts (both guarantee-style and compute_cases-style), Verus-compatible enum dispatch using pattern matching, and requirements traceability with change reports.
