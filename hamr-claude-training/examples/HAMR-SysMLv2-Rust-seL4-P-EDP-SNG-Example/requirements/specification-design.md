# Claude Instructions for Specification Design

I'm designing a class project to illustrate how to 
 - decompose natural system requirements to natural language HAMR component requirements
 - write GUMBO specifications to formally specify component natural language requirements
 - for technical aspects of GUMBO specifications, I want to illustrate GUMBO specifications in the context of the periodic components with event data ports as found in the Simple Network Guard models in @../sysmlv2

The students will 
 - apply HAMR SysMLv2 Type Checking, HAMR SysMLv2 Logika Check in the CodeIVE to check model-level specifications.
 - run HAMR code generation to update code-level Verus contracts and GUMBOX testing infrastructure
 - use Verus to verify that the gate and filter components satisfy their specifications
 - use HAMR testing, including GUMBOX property-based testing infrastructure to test that the gate and filter components satisfy their specifications.  Testing should include some manual GUMBOX tests to provide validation of the specifications
 (i.e., do the specifications match the stakeholders mental model fo the system).
 
For the decomposition of natural language requirements to HAMR component requirements, illustrate..
 - writing natural language requirements in a structured way to faciliate the eventual mapping to the GUMBO specifications
 - traceability from the component requirements back to the system requirements
 - construction of a "glossary" that provides a canonical application domain terminology with associated definitions

For the GUMBO specifications, here are some key features that I would like to illustrate:
 - use of GUMBO library functions to specify the policies associated with gate and filter components
 - use GUMBO "guarantee" clauses only for the gate component compute contract to illustrate how the use of the "implies" key word can give the same semantics as GUMBO case assume / guarantee pairs
 - use of the HasEvent and NoEvent GUMBO constructs to reason about the presence or absence of messages on ports
 - use of the short circuit GUMBO "and" operation and HasEvent to guard other expressions that access port messages (avoiding erroneous access of an empty port)
 - use of GUMBO library functions in integration constraints to summarize important invariants that should hold on ports.
   These same GUMBO library functions may also be used in GUMBO compute and initialize specifications.

Claude tasks
- following documented HAMR process steps, develop a model solution illustrating the above concepts based on the existing Simple Network Guard (SNG) requirements, models, and code.
- if possible the model solution should avoid changing the model-level component definitions (only add new GUMBO features)
- in the process of building the model solution, Claude should report if adding the specifications, testing, and verification uncovered any errors in the application code of the gate and filter components.

Resources
- For illustrations of GUMBO Libraries, etc. in addition to the material in hamr-claude-training you may find the following useful:
  - Use of GUMBO specs for event-data ports in an application similar to SNG: https://github.com/loonwerks/INSPECTA-models/blob/main/open-platform-models/isolate-ethernet-simple/sysml/SW.sysml
  - Use of GUMBO library declarations: https://github.com/loonwerks/INSPECTA-Open-Platform/tree/main/ardupilot-basic, 
  in particular the file https://github.com/loonwerks/INSPECTA-Open-Platform/blob/main/ardupilot-basic/GumboLib.sysml shows how policies regarding message guarding are formally specified and hierarchically decomposed as GUMBO functions
  - HAMR SysMLv2 and GUMBO Quick Reference: https://github.com/santoslab/software-specs/blob/master/sysmlv2-reference.md


  
