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

## Examples

Examples are in the folder @examples. The purpose and features illustrated in each example are in @examples/README.md.  The examples illustrate:

  - the "HAMR SysML" subset of the SysMLv2 modeling language that HAMR supports as well as model patterns that are commonly used
  - output produced by HAMR
  - application code built by developers and how the code is integrated with HAMR-auto-generated code.

# Other Information


MCP information that should apply to any HAMR project is in `.mcp.json`




