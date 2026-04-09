# Purpose

This folder provides context information to inform Claude in helping software engineers apply the HAMR model driven development toolchain.  This context material includes HAMR documentation and example systems.  

# Material Organization

## Documentation 

Documentation is in the folder @doc.  The primary contents are as follows:

- HAMR Overview - @doc/hamr-overview.md contains a summary of HAMR's overall purpose, nature of its input and output artifacts, and develop workflows.
- Project Folder Structure - @doc/folder-structure.md contains a summary of the folder structure that every HAMR project follows
- Technical Approach - @doc/technical-approach.md explains the semantics of HAMR thread component execution.
- Use of AADL Concepts in SysMLv2 - @doc/sysmlv2-aadl-concepts.md illustrates how the SysMLv2 AADL libraries are used to represent AADL concepts in SysMLv2.  It also provides some intuition about how each of these concepts are interpresented in seL4 Microkit code generation.
- MCP Tools Reference - @doc/mcp-tools.md documents the Sireum MCP tools available for HAMR development, including type checking, code generation, configuration, and formal verification.
- Modeling Tips - @doc/modeling-tips.md contains practical tips and known issues for SysMLv2 modeling with HAMR, including keyword conflicts, architecture constraints, GUMBO gotchas, and common patterns.
- Component Implementation Guide - @doc/component-implementation-guide.md explains how to implement Rust application logic for HAMR thread components, including Verus blocks, port API usage, logging, and complete examples.
- Testing Guide - @doc/testing-guide.md describes how to test HAMR components using the generated test infrastructure, covering manual unit tests, GUMBOX contract-based tests, and automated PropTest property-based tests.
- Build and Verification Commands - @doc/build-and-verification-commands.md documents the Makefile targets for building, testing, and running Verus verification on component crates.
- SysMLv2 and GUMBO Quick Reference - @doc/sysmlv2-gumbo-quick-reference.md provides a compact syntax reference for SysMLv2 modeling constructs and the GUMBO contract language, including package declarations, component definitions, port declarations, data types, GUMBO specification blocks, integration constraints, compute contracts, library functions, and expression syntax.
- GUMBO BNF Reference - @doc/gumbo-bnf.md provides a formal BNF grammar for the GUMBO contract language (SysMLv2 version), derived from the ANTLR parser grammar and augmented with built-in predicates (HasEvent, NoSend, MustSend, In) and naming conventions not visible in the grammar.

## Examples

Examples are in the folder @examples. The purpose and features illustrated in each example are in @examples/README.md.  The examples illustrate:

  - the "HAMR SysML" subset of the SysMLv2 modeling language that HAMR supports as well as model patterns that are commonly used
  - output produced by HAMR
  - application code built by developers and how the code is integrated with HAMR-auto-generated code.

The Simple Network Guard (SNG) example (@examples/HAMR-SysMLv2-Rust-seL4-P-EDP-SNG-Example) additionally demonstrates GUMBO contracts (integration constraints and compute contracts), event data port filtering/dropping patterns, and requirements traceability with change reports in its `reports/` folder.

# Other Information


MCP information that should apply to any HAMR project is in `.mcp.json`




